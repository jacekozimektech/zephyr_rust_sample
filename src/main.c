/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <stdint.h>
#include "zephyr_rust.h"

void main(void)
{
	printk("Hello World! Rust %s\n", CONFIG_BOARD);
	push_to_fifo(100);
	printk("Len: %d\n", get_fifo_len());
	push_to_fifo(100);
	printk("Len: %d\n", get_fifo_len());
	push_to_fifo(100);
	push_to_fifo(100);
	push_to_fifo(150);
	push_to_fifo(50);
	printk("Len: %d\n", get_fifo_len());
	printk("Mean: %d\n", (int)get_fifo_mean());
}
