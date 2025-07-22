
/* backup.h -- declarations for making Emacs style backup file names
   Copyright (C) 1992 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it without restriction.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  */


/* When to make backup files.  Analagous to 'version-control'
   in Emacs. */
enum backup_mode
{
  /* Uninitialized or indeterminate value */
  unknown,

  /* Never make backups. */
  none,

  /* Make simple backups of every file. */
  simple,

  /* Make numbered backups of files that already have numbered backups,
     and simple backups of the others. */
  numbered_existing,

  /* Make numbered backups of every file. */
  numbered
};

struct version_control_values
{
  enum backup_mode value;
  char *name;
};

/* Determine the value of `version_control' by looking in the
   environment variable "VERSION_CONTROL".  Defaults to
   numbered_existing. */
extern enum backup_mode version_control_value ();

/* Initialize information used in determining backup filenames. */
extern void initialize_backups ();

/* Make a backup copy of FILE, taking into account version-control.
   See the description at the beginning of the file for details. */
extern void make_backup ();
