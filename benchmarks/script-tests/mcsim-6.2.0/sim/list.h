/* list.h

   Written by D.R.Maszle
   28 October 1991

   Copyright (c) 1991-2017 Free Software Foundation, Inc.

   This file is part of GNU MCSim.

   GNU MCSim is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License
   as published by the Free Software Foundation; either version 3
   of the License, or (at your option) any later version.

   GNU MCSim is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GNU MCSim; if not, see <http://www.gnu.org/licenses/>
*/

#ifndef LIST_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions  */

#include "hungtype.h"


/* ----------------------------------------------------------------------------
   Typedefs  */

typedef struct tagLISTELEM {   /* List element record */
  PVOID pData;                 /* User data, to be recast */
  struct tagLISTELEM *pleNext; /* Next in the list */
} LISTELEM, *PLISTELEM; /* tagLISTELEM */


typedef struct tagLIST { /* List header record */
  PLISTELEM pleHead;     /* First elem in list */
  PLISTELEM pleTail;     /* Last elem in list */
  int    iSize;          /* Number of elems in list */
} LIST, *PLIST; /* tagLIST */

/* Callback function for ForAllList, ForAllList3 */
typedef int (*PFI_FORLISTCALLBACK) (PVOID pData, PVOID pUserInfo);
typedef void (*PFI_FORLISTCALLBACK3) (PVOID pData, PVOID pUserInfo1,
                                     PVOID pUserInfo2, PVOID pUserInfo3);

/* Callback for FreeList() */
typedef void (*PFV_FREELISTCALLBACK) (PVOID pData);

/* ----------------------------------------------------------------------------
   Macros  */

#define ListLength(plist) ((plist) ? (plist)->iSize : 0)


/* ----------------------------------------------------------------------------
   Prototypes  */


int  ForAllList (PLIST plist, PFI_FORLISTCALLBACK pfiForAllData, PVOID pInfo);
void ForAllList3 (PLIST plist, PFI_FORLISTCALLBACK3 pfiCallback,
                  PVOID pUserInfo1, PVOID pUserInfo2, PVOID pUserInfo3);

void FreeList(PLIST *pplist, PFV_FREELISTCALLBACK pfvFreeData, BOOL bAndData);

PLIST InitList (void);

void QueueListItem (PLIST plist, PVOID pData);

#define LIST_H_DEFINED
#endif

/* end */

