#include <linux/module.h>
#include <linux/fs.h>
#include <linux/cdev.h>
#include <linux/device.h>
#include <linux/platform_device.h>
#include <linux/dma-mapping.h>

#define DRIVER_NAME "dma_mmap_driver"
#define DEVICE_NAME "dma_mmap_device"
#define CLASS_NAME  "dma_mmap_class"
#define BUFFER_SIZE 4096

static struct class *dma_mmap_class;    // 클래스 포인터
static struct cdev cdev;                // 문자 장치 구조체
static dev_t dev_num;                   // 장치 번호
static void *dma_buffer;                // DMA 버퍼 포인터
static dma_addr_t dma_handle;           // DMA 핸들
static struct platform_device *pdev;    // 플랫폼 장치 포인터

// 가상의 DMA 인터럽트 핸들러 함수
void virtual_dma_interrupt_handler(void) {
    pr_info("DMA operation completed. Data: %s\n", (char*)dma_buffer);
}

// 가상의 DMA 전송 함수
void perform_virtual_dma_transfer(const char *data) {
    strcpy(dma_buffer, data);
    virtual_dma_interrupt_handler();
}

static int dma_mmap_open(struct inode *inode, struct file *file) {
    const char *sample_data = "Hello, DMA!";
    perform_virtual_dma_transfer(sample_data);
    return 0;
}

static int dma_mmap_mmap(struct file *filp, struct vm_area_struct *vma) {
    return dma_mmap_coherent(&pdev->dev, vma, dma_buffer, dma_handle, BUFFER_SIZE);
}

// 파일 연산 정의
static struct file_operations fops = {
    .open = dma_mmap_open,
    .mmap = dma_mmap_mmap,
};

// 플랫폼 장치 probe 함수
static int dma_mmap_probe(struct platform_device *pdev) {
    // 여기서 장치별 초기화를 수행할 수 있음
    return 0;
}

// 플랫폼 드라이버 정의
static struct platform_driver dma_mmap_platform_driver = {
    .probe = dma_mmap_probe,
    .driver = {
        .name = DRIVER_NAME,
    },
};

static int __init dma_mmap_init(void) {
    int ret;

    // 플랫폼 드라이버 등록
    ret = platform_driver_register(&dma_mmap_platform_driver);
    if (ret) {
        pr_err("Failed to register platform driver\n");
        return ret;
    }

    // 플랫폼 장치 생성 및 등록
    pdev = platform_device_register_simple(DRIVER_NAME, -1, NULL, 0);
    if (IS_ERR(pdev)) {
        pr_err("Failed to register platform device\n");
        platform_driver_unregister(&dma_mmap_platform_driver);
        return PTR_ERR(pdev);
    }

    // 문자 장치 할당 및 설정
    alloc_chrdev_region(&dev_num, 0, 1, DRIVER_NAME);
    cdev_init(&cdev, &fops);
    cdev_add(&cdev, dev_num, 1);

    dma_mmap_class = class_create(THIS_MODULE, CLASS_NAME);
    device_create(dma_mmap_class, NULL, dev_num, NULL, DEVICE_NAME);

    // DMA 버퍼 할당
    dma_buffer = dma_alloc_coherent(&pdev->dev, BUFFER_SIZE, &dma_handle, GFP_KERNEL);
    if (!dma_buffer) {
        pr_err("Failed to allocate DMA buffer\n");
        // 오류 발생 시 자원 해제
        device_destroy(dma_mmap_class, dev_num);
        class_destroy(dma_mmap_class);
        cdev_del(&cdev);
        unregister_chrdev_region(dev_num, 1);
        platform_device_unregister(pdev);
        platform_driver_unregister(&dma_mmap_platform_driver);
        return -ENOMEM;
    }

    pr_info("DMA mmap driver initialized\n");
    return 0;
}

static void __exit dma_mmap_exit(void) {
    // 모듈 종료 시 자원 해제
    dma_free_coherent(&pdev->dev, BUFFER_SIZE, dma_buffer, dma_handle);
    device_destroy(dma_mmap_class, dev_num);
    class_destroy(dma_mmap_class);
    cdev_del(&cdev);
    unregister_chrdev_region(dev_num, 1);
    platform_device_unregister(pdev);
    platform_driver_unregister(&dma_mmap_platform_driver);
    pr_info("DMA mmap driver exited\n");
}

module_init(dma_mmap_init);
module_exit(dma_mmap_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("RustWithLinux");
MODULE_DESCRIPTION("DMA mmap example driver with platform device");