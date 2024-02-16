#include "asm/page.h"
#include "linux/blk_types.h"
#include "linux/sysfb.h"
#include <linux/module.h>
#include <linux/blkdev.h>
#include <linux/blk-mq.h>
#include <linux/idr.h>

// 설정된 기본 변수들
unsigned long capacity_mb = 40;
unsigned long max_segments = 32;
unsigned long max_segment_size = 65536;
unsigned long lbs = PAGE_SIZE;
unsigned long pbs = PAGE_SIZE;

// RAM 기반의 블록 장치를 나타내는 구조체
struct blk_ram_dev_t {
        sector_t capacity;
        u8 *data;
        struct blk_mq_tag_set tag_set;
        struct gendisk *disk;
};

// 정적 변수들
static int major;
static DEFINE_IDA(blk_ram_indexes);
static struct blk_ram_dev_t *blk_ram_dev = NULL;

// 요청을 처리하는 함수
static blk_status_t blk_ram_queue_rq(struct blk_mq_hw_ctx *hctx,
                                     const struct blk_mq_queue_data *bd)
{
        struct request *rq = bd->rq;
        blk_status_t err = BLK_STS_OK;
        struct bio_vec bv;
        struct req_iterator iter;
        loff_t pos = blk_rq_pos(rq) << SECTOR_SHIFT;
        struct blk_ram_dev_t *blkram = hctx->queue->queuedata;
        loff_t data_len = (blkram->capacity << SECTOR_SHIFT);

        blk_mq_start_request(rq);

        rq_for_each_segment(bv, rq, iter) {
                unsigned int len = bv.bv_len;
                void *buf = page_address(bv.bv_page) + bv.bv_offset;

                // 데이터의 길이를 초과하는 경우 에러 처리
                if (pos + len > data_len) {
                        err = BLK_STS_IOERR;
                        break;
                }

                // 읽기와 쓰기 연산 처리
                switch (req_op(rq)) {
                case REQ_OP_READ:
                        memcpy(buf, blkram->data + pos, len);
                        break;
                case REQ_OP_WRITE:
                        memcpy(blkram->data + pos, buf, len);
                        break;
                default:
                        err = BLK_STS_IOERR;
                        goto end_request;
                }
                pos += len;
        }

end_request:
        blk_mq_end_request(rq, err);
        return BLK_STS_OK;
}

// 블록 큐 연산 정의
static const struct blk_mq_ops blk_ram_mq_ops = {
        .queue_rq = blk_ram_queue_rq,
};

// 블록 장치 연산 정의
static const struct block_device_operations blk_ram_rq_ops = {
        .owner = THIS_MODULE,
};

// 모듈 초기화 함수
static int __init blk_ram_init(void)
{
        int ret = 0;
        int minor;
        struct gendisk *disk;
        loff_t data_size_bytes = capacity_mb << 20;

        // 블록 장치 등록
        ret = register_blkdev(0, "blkdriver");
        if (ret < 0)
                return ret;

        major = ret;
        blk_ram_dev = kzalloc(sizeof(struct blk_ram_dev_t), GFP_KERNEL);

        // 메모리 할당 실패 처리
        if (blk_ram_dev == NULL) {
                pr_err("memory allocation failed for blk_ram_dev\n");
                ret = -ENOMEM;
                goto unregister_blkdev;
        }

        blk_ram_dev->capacity = data_size_bytes >> SECTOR_SHIFT;
        blk_ram_dev->data = kvmalloc(data_size_bytes, GFP_KERNEL);

        // RAM 디스크 메모리 할당 실패 처리
        if (blk_ram_dev->data == NULL) {
                pr_err("memory allocation failed for the RAM disk\n");
                ret = -ENOMEM;
                goto data_err;
        }

        // 태그 셋 초기화 및 설정
        memset(&blk_ram_dev->tag_set, 0, sizeof(blk_ram_dev->tag_set));
        blk_ram_dev->tag_set.ops = &blk_ram_mq_ops;
        blk_ram_dev->tag_set.queue_depth = 128;
        blk_ram_dev->tag_set.numa_node = NUMA_NO_NODE;
        blk_ram_dev->tag_set.flags = BLK_MQ_F_SHOULD_MERGE;
        blk_ram_dev->tag_set.cmd_size = 0;
        blk_ram_dev->tag_set.driver_data = blk_ram_dev;
        blk_ram_dev->tag_set.nr_hw_queues = 1;

        ret = blk_mq_alloc_tag_set(&blk_ram_dev->tag_set);
        if (ret)
                goto data_err;

        // 디스크 할당
        disk = blk_ram_dev->disk =
                blk_mq_alloc_disk(&blk_ram_dev->tag_set, blk_ram_dev);

        // 큐 설정
        blk_queue_logical_block_size(disk->queue, lbs);
        blk_queue_physical_block_size(disk->queue, pbs);
        blk_queue_max_segments(disk->queue, max_segments);
        blk_queue_max_segment_size(disk->queue, max_segment_size);

        // 디스크 할당 실패 처리
        if (IS_ERR(disk)) {
                ret = PTR_ERR(disk);
                pr_err("Error allocating a disk\n");
                goto tagset_err;
        }

        // 파티션 및 더 많은 RAM 장치 지원은 필요 없음
        minor = ret = ida_alloc(&blk_ram_indexes, GFP_KERNEL);
        if (ret < 0)
                goto cleanup_disk;

        // 디스크 정보 설정
        disk->major = major;
        disk->first_minor = minor;
        disk->minors = 1;
        snprintf(disk->disk_name, DISK_NAME_LEN, "blkram");
        disk->fops = &blk_ram_rq_ops;
        disk->flags = GENHD_FL_NO_PART;
        set_capacity(disk, blk_ram_dev->capacity);

        ret = add_disk(disk);
        if (ret < 0)
                goto cleanup_disk;

        pr_info("module loaded\n");
        return 0;

cleanup_disk:
        put_disk(blk_ram_dev->disk);
tagset_err:
        kfree(blk_ram_dev->data);
data_err:
        kfree(blk_ram_dev);
unregister_blkdev:
        unregister_blkdev(major, "blkram");

        return ret;
}

// 모듈 종료 함수
static void __exit blk_ram_exit(void)
{
        if (blk_ram_dev->disk) {
                del_gendisk(blk_ram_dev->disk);
                put_disk(blk_ram_dev->disk);
        }
        unregister_blkdev(major, "blkram");
        kfree(blk_ram_dev);

        pr_info("module unloaded\n");
}

module_init(blk_ram_init);
module_exit(blk_ram_exit);

MODULE_LICENSE("GPL");