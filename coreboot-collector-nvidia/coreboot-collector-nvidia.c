// SPDX-License-Identifier: GPL-2.0+
/*
 * System76 ACPI Driver
 *
 * Copyright (C) 2019 System76
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

#include <linux/acpi.h>
#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/pci.h>
#include <linux/types.h>

static union acpi_object * dump_object(union acpi_object * obj) {
	int i;
	switch (obj->type) {
		case ACPI_TYPE_INTEGER:
			pr_info("Integer: 0x%llx\n", obj->integer.value);
			break;
		case ACPI_TYPE_STRING:
			pr_info("String: '%s'\n", obj->string.pointer);
			break;
		case ACPI_TYPE_BUFFER:
			pr_info("Buffer (0x%x) {", obj->buffer.length);
			for (i = 0; i < obj->buffer.length; i++) {
				if (i > 0) {
					pr_cont(", ");
				}
				pr_cont("0x%02X", obj->buffer.pointer[i]);
			}
			pr_cont("}\n");
			break;
		default:
			pr_info("Unsupported type: 0x%x\n", obj->type);
			break;
	}
	return obj;
}

static union acpi_object * nvidia_dsm(acpi_handle handle, const guid_t * guid, u64 rev, u64 func, char arg[4]) {
	union acpi_object argv4 = {
		.buffer.type = ACPI_TYPE_BUFFER,
		.buffer.length = 4,
		.buffer.pointer = arg
	};
	return acpi_evaluate_dsm(
		handle,
		guid,
		rev,
		func,
		&argv4
	);
}

static const guid_t NBCI_DSM_GUID = GUID_INIT(
	0xD4A50B75, 0x65C7, 0x46F7, 0xBF, 0xB7, 0x41, 0x51, 0x4C, 0xEA, 0x02, 0x44
);
#define NBCI_REVISION_ID 0x0102
#define NBCI_FUNC_SUPPORT 0
#define NBCI_FUNC_GETOBJBYTYPE 16
static union acpi_object * nbci(acpi_handle handle, u64 func, char arg[4]) {
	return nvidia_dsm(
		handle,
		&NBCI_DSM_GUID,
		NBCI_REVISION_ID,
		func, 
		arg
	);
}

static void cbcnv_handle(acpi_handle handle) {
	union acpi_object *obj = NULL;
	char arg[4] = { 0, 0, 0, 0 };

	pr_info("coreboot-collector-nvidia handle %p\n", handle);

	pr_info("NBCI_FUNC_SUPPORT\n");
	obj = nbci(handle, NBCI_FUNC_SUPPORT, arg);
	if (obj) {
		ACPI_FREE(dump_object(obj));
		obj = NULL;
	} else {
		acpi_handle_info(handle, "failed to execute\n");
	}

	arg[0] = 0;
	arg[1] = 0;
	arg[2] = 'R';
	arg[3] = 'D';
	pr_info("NBCI_FUNC_GETOBJBYTYPE '%c%c'\n", arg[3], arg[2]);
	obj = nbci(handle, NBCI_FUNC_GETOBJBYTYPE, arg);
	if (obj) {
		ACPI_FREE(dump_object(obj));
		obj = NULL;
	} else {
		acpi_handle_info(handle, "failed to execute\n");
	}

	arg[0] = 0;
	arg[1] = 0;
	arg[2] = 'K';
	arg[3] = 'V';
	pr_info("NBCI_FUNC_GETOBJBYTYPE '%c%c'\n", arg[3], arg[2]);
	obj = nbci(handle, NBCI_FUNC_GETOBJBYTYPE, arg);
	if (obj) {
		ACPI_FREE(dump_object(obj));
		obj = NULL;
	} else {
		acpi_handle_info(handle, "failed to execute\n");
	}
}

static int __init cbcnv_init(void) {
    struct pci_dev *pdev = NULL;
	acpi_handle handle;

	pr_info("coreboot-collector-nvidia init\n");

    while ((pdev = pci_get_device(PCI_ANY_ID, PCI_ANY_ID, pdev)) != NULL) {
        int pci_class = pdev->class >> 8;
        if (pci_class != PCI_CLASS_DISPLAY_VGA
		 && pci_class != PCI_CLASS_DISPLAY_3D) {
            continue;
		}

		if (pdev->vendor != PCI_VENDOR_ID_NVIDIA) {
			continue;
		}

		pr_info("NVIDIA GPU at '%s'\n", dev_name(&pdev->dev));

        handle = ACPI_HANDLE(&pdev->dev);
		if (!handle) {
			pr_err("failed to find ACPI handle\n");
			continue;
		}
		
		cbcnv_handle(handle);
	}

	return 0;
}

static void __exit cbcnv_exit(void) {
	pr_info("coreboot-collector-nvidia exit\n");
}

module_init(cbcnv_init);
module_exit(cbcnv_exit);

MODULE_DESCRIPTION("Coreboot Collector NVIDIA");
MODULE_AUTHOR("Jeremy Soller <jeremy@system76.com>");
MODULE_LICENSE("GPL");
