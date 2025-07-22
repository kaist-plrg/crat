/* list.c

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

   A LIST data type utility.
*/

#include <stdlib.h>

#include "lexerr.h"
#include "list.h"


/* -----------------------------------------------------------------------------
   InitList

   Initialized a list header structure and returns the pointer.  This
   LIST may then be used in other exciting list functions.
*/

PLIST InitList (void)
{
  /* Allocate header record */
  PLIST plist = (PLIST) malloc (sizeof(LIST));

  if (plist) {                /* If allocation succeeds, */
    plist->pleHead = NULL;    /* .. initialize all to 0  */
    plist->pleTail = NULL;
    plist->iSize = 0;
  }
  else
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "InitList", NULL);

  return (plist);

} /* InitList */


/* -----------------------------------------------------------------------------
   FreeList

   Frees the memory allocated to a LIST, incrementing through each
   element sequentially. Allows user to free data with a provided
   callback function, or to specify free'ing of data in this routine.

   If the pfvFreeData function is non-NULL, the function is called
   with each data pointer in the list, and bAndData is ignored.

   If the callback is NULL and the boolean bAndData is set, free() is
   called with each user data pointer before freeing the list element.

   NOTE: ppList is a pointer to a LIST structure which is itself a pointer.
*/

void FreeList (PLIST *pplist, PFV_FREELISTCALLBACK pfvFreeData, BOOL bAndData)
{
  PLIST plist = *pplist; /* Get single pointer to work with */

  if (!plist)
    return;

  while (plist->pleHead) {
    if (pfvFreeData)
      (*pfvFreeData)(plist->pleHead->pData); /* Call user free routine */

    else if (bAndData && plist->pleHead->pData)
      free (plist->pleHead->pData); /* Free user data block */

    plist->pleTail = plist->pleHead; /* Save p to current elem */
    plist->pleHead = plist->pleHead->pleNext; /* Next list item */
    free (plist->pleTail); /* Free saved current elem */

  } /* while */

  free (plist); /* Free the list header */
  *pplist = NULL; /* Reset the user's pointer */

} /* FreeList */


/* -----------------------------------------------------------------------------
   QueueListItem

   Adds a new list item to the tail of the list.
*/

void QueueListItem (PLIST plist, PVOID pData)
{
  PLISTELEM pNewElem;

  if (!plist)
    return;

  if (!(pNewElem = (PLISTELEM) malloc (sizeof(LISTELEM))))
    ReportError (NULL, RE_OUTOFMEM | RE_FATAL, "QueueListItem", NULL);

  pNewElem->pData = pData; /* Init new element */
  pNewElem->pleNext = NULL;

  if (plist->pleTail)
    plist->pleTail->pleNext = pNewElem; /* Link new element */
  else
    plist->pleHead = pNewElem; /* Link first element */

  plist->pleTail = pNewElem; /* Reset tail to new elem */
  plist->iSize++; /* Increment size of list */

} /* QueueListItem */


/* -----------------------------------------------------------------------------
   ForAllList

   Takes a PLIST, a PFI_LISTCALLBACK function, and PVOID to user
   information, and calls the callback function for each of the list
   elems in sequence.

   The PFI_LISTCALLBACK takes the user's data point at each list
   element and the pointer to user information as arguments.  It
   returns an integer.  The sum of the integers of all calls to the
   callback is returned from ForAllList().
*/

int ForAllList (PLIST plist, PFI_FORLISTCALLBACK pfiCallback, PVOID pUserInfo)
{
  int iTotal = 0;
  PLISTELEM ple;

  if (!plist || !pfiCallback)
    return 0;

  ple = plist->pleHead;

  while (ple) {
    iTotal += (*pfiCallback) (ple->pData, pUserInfo);
    ple = ple->pleNext; /* Increment to next elem */
  }

  return (iTotal);

} /* ForAllList */

/* -----------------------------------------------------------------------------
   ForAllList3

   Like ForAllList, but takes three user arguments
   Included temporarily to avoid rewriting a lot of code
*/

void ForAllList3(PLIST plist, PFI_FORLISTCALLBACK3 pfiCallback,
                 PVOID pUserInfo1, PVOID pUserInfo2, PVOID pUserInfo3)
{
  PLISTELEM ple;

  if (!plist || !pfiCallback)
    return;

  ple = plist->pleHead;

  while (ple) {
    (*pfiCallback) (ple->pData, pUserInfo1, pUserInfo2, pUserInfo3);
    ple = ple->pleNext; /* Increment to next elem */
  }

} /* ForAllList3 */
