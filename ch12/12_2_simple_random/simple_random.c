#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/random.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Your Name");
MODULE_DESCRIPTION("A simple /dev/random-like kernel module");
MODULE_VERSION("0.1");

#define DEVICE_NAME "simple_random"

static int major_num;
static char rand_num = 0;
static int device_open_count = 0;

static int device_open(struct inode *inode, struct file *file) {
    if (device_open_count)
        return -EBUSY;

    device_open_count++;
    try_module_get(THIS_MODULE);
    return 0;
}

static int device_close(struct inode *inode, struct file *file) {
    device_open_count--;
    module_put(THIS_MODULE);
    return 0;
}

static ssize_t device_read(struct file *filp, char *buffer, size_t len, loff_t *offset) {
    get_random_bytes(&rand_num, sizeof(rand_num));
    if (*offset == 0) {
        if (copy_to_user(buffer, &rand_num, sizeof(rand_num)) != 0) {
            return -EFAULT;
        }
        *offset += 1;
        return sizeof(rand_num);
    } else {
        return 0;
    }
}

static struct file_operations file_ops = {
    .read = device_read,
    .open = device_open,
    .release = device_close
};

static int __init simple_random_init(void) {
    major_num = register_chrdev(0, DEVICE_NAME, &file_ops);

    if (major_num < 0) {
        printk(KERN_ALERT "Could not register device: %d\n", major_num);
        return major_num;
    }

    printk(KERN_INFO "simple_random module loaded with device major number %d\n", major_num);
    return 0;
}

static void __exit simple_random_exit(void) {
    unregister_chrdev(major_num, DEVICE_NAME);
    printk(KERN_INFO "Goodbye, simple_random!\n");
}

module_init(simple_random_init);
module_exit(simple_random_exit);
