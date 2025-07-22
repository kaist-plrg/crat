/* sim.h

   Originally written by Don Maszle

   Copyright (c) 1993-2020. Free Software Foundation, Inc.

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

   Header file for simulation

   Modified the obsolete "Options Flags"

*/

#ifndef SIM_H_DEFINED

/* ----------------------------------------------------------------------------
   Inclusions
*/

#include "modiface.h"
#include "list.h"
#include "matutil.h"
#include "random.h"


/* ----------------------------------------------------------------------------
   Constants
*/

/* Version and copyright */
#define VSZ_VERSION "v6.2.0"
#define VSZ_COPYRIGHT "Copyright (c) 1993-2020 Free Software Foundation, Inc."

/* These are potential array size problems.
   Other maximum sizes are MAX_EQN and MAX_LEX in lex.h */

#define MAX_LEVELS            10    /* for now; actual possible is 255 */
#define MAX_INSTANCES         20000 /* allowable instances of `Level' at any
                                       depth and max number of experiments */
#define LSODES_IWORKSIZE      1000  /* Lsodes tells if it needs more */
#define LSODES_RWORKSIZE      60000 /* Idem */
#define ARGS_MAX              8     /* Maximum number of args to lex */
                                    /* Used in allocating argument strings */
#define MAX_PRINT_VARS        200   /* Arbitrary */
#define MAX_FILENAMESIZE      200   /* Arbitrary */

/* Keyword Map constants */

#define KM_INTEGRATE       1
#define KM_SIMULATE        2
#define KM_STARTTIME       3
#define KM_TEMPERATURE     4
#define KM_PRINT           5
#define KM_PRINTSTEP       6
#define KM_DATA            7

#define KM_SIMTYPE         8  /* For simulation type specification */
#define KM_DEFAULTSIM      9  /* 'Normal' keyword: normal sims */

#define KM_EXPERIMENT      10
#define KM_MONTECARLO      11
#define KM_MCVARY          12
#define KM_SETPOINTS       13
#define KM_OUTPUTFILE      14
#define KM_MCMC            15
#define KM_LEVEL           16
#define KM_OPTDESIGN       17

#define KM_END             100

/* Function argument keywords */

#define KM_YES             200
#define KM_NO              201

#define KM_UNIFORM         210
#define KM_LOGUNIFORM      211
#define KM_BETA            212
#define KM_NORMAL          213
#define KM_LOGNORMAL       214
#define KM_TRUNCNORMAL     215
#define KM_TRUNCLOGNORMAL  216
#define KM_CHI2            217
#define KM_BINOMIAL        218
#define KM_PIECEWISE       219
#define KM_EXPONENTIAL     220
#define KM_GGAMMA          221
#define KM_POISSON         222
#define KM_INVGGAMMA       223
#define KM_NORMALV         224
#define KM_LOGNORMALV      225
#define KM_TRUNCNORMALV    226
#define KM_TRUNCLOGNORMALV 227
#define KM_BINOMIALBETA    228
#define KM_HALFNORMAL      229
#define KM_TRUNCINVGGAMMA  230
#define KM_GENLOGNORMAL    231    
#define KM_STUDENTT        232
#define KM_CAUCHY          233
#define KM_HALFCAUCHY      234
#define KM_NORMALCV        235
#define KM_TRUNCNORMALCV   236
#define KM_USERLL          237
#define KM_NEGATIVEBINOM   238

#define KM_PREDICTION      300

#define KM_LSODES          600
#define KM_CVODES          601
#define KM_EULER           602

#define KM_FORWARD         700
#define KM_BACKWARD        701

#define KM_REPLACE         710
#define KM_ADD             711
#define KM_MULTIPLY        712

/* Context bit flags */

#define CN_END             0x0000    /* No context */

#define CN_GLOBAL          0x0001
#define CN_EXPERIMENT      0x0002
#define CN_FUNCARG         0x0100

#define CN_ALL             0xFFFF    /* All contexts */

/* Analysis Types */

#define AT_NOTSPECD     0    /* Not yet specified */
#define AT_DEFAULTSIM   1    /* Normal simulation */
#define AT_MONTECARLO   2    /* Monte Carlo variations */
#define AT_SETPOINTS    3    /* Set points simulation */
#define AT_MCMC         4    /* Metropolis or Gibbs estimation */
#define AT_OPTDESIGN    5    /* Optimal design search */

/* Monte Carlo Variation types */

#define MCV_SETPOINTS       (-1) /* Not really Monte Carlo */
#define MCV_UNIFORM         0
#define MCV_LOGUNIFORM      1
#define MCV_BETA            2
#define MCV_NORMAL          3
#define MCV_LOGNORMAL       4
#define MCV_TRUNCNORMAL     5
#define MCV_TRUNCLOGNORMAL  6
#define MCV_CHI2            7
#define MCV_BINOMIAL        8
#define MCV_PIECEWISE       9
#define MCV_EXPONENTIAL     10
#define MCV_GGAMMA          11
#define MCV_POISSON         12
#define MCV_INVGGAMMA       13
#define MCV_NORMALV         14
#define MCV_LOGNORMALV      15
#define MCV_TRUNCNORMALV    16
#define MCV_TRUNCLOGNORMALV 17
#define MCV_BINOMIALBETA    18
#define MCV_HALFNORMAL      19
#define MCV_TRUNCINVGGAMMA  20
#define MCV_GENLOGNORMAL    21
#define MCV_STUDENTT        22
#define MCV_CAUCHY          23
#define MCV_HALFCAUCHY      24
#define MCV_NORMALCV        25
#define MCV_TRUNCNORMALCV   26
#define MCV_USERLL          27
#define MCV_NEGATIVEBINOM   28

/* Integration Method types */

#define IAL_EULER  2  /* Euler  algorithm */
#define IAL_LSODES 3  /* lsodes algorithm */
#define IAL_CVODES 4  /* cvodes algorithm */

/* Integrator spec defaults */

#define IAL_DEFAULT     IAL_LSODES
#define IOPT_DEFAULT    (0)
#define ITOL_DEFAULT    (1)
#define ITASK_DEFAULT   (4)   /* do not overshoot - FB 01/07/97 */
#define RTOL_DEFAULT    (1.0e-5)
#define ATOL_DEFAULT    (1.0e-7)
#define IMF_DEFAULT     (222) /* stiff */
#define MXSTEPS_DEFAULT (500)
#define MAXNEF_DEFAULT  (7)
#define MAXCOR_DEFAULT  (3)
#define MAXNCF_DEFAULT  (10)
#define NLSCOEF_DEFAULT (0.1)
#define TSTEP_DEFAULT   (1)

/* Simulation specification defaults */

#define T0_DEFAULT            0.0
#define TFINAL_DEFAULT        0.0
#define NSIMULATIONS_DEFAULT  0

/* Defs for Distrib statement */

#define MCVP_FIXD     0
#define MCVP_PARM     1
#define MCVP_PRED     2
#define MCVP_DATA     3


/* ----------------------------------------------------------------------------
   Typedefs
*/

/* Union of two types of variables: constants and input fns */

typedef union tagUVAR {
  double dVal;
  PIFN pifn;
} UVAR; /* tagUVAR */


/* Modification specification for one variable */

typedef struct tagVARMODIFICATION {
  HVAR hvar; /* Handle to the variable */
  UVAR uvar; /* Union of variable value or input function spec */
} VARMODIFICATION, *PVARMOD; /* tagVARMODIFICATION */


/* Specification of integrator settings */

typedef struct tagINTSPEC {
  int     iAlgo;          /* one of IM_ types */

  double  dRtol;          /* relative error tolerance */
  double  dAtol;          /* absolute error tolerance */

  /* Lsodes specifics */ 
  long    iopt;           /* optional inputs flag */
  long    itask;          /* type of work */
  long    itol;           /* type of error checking */
  long    iMf;            /* 0: nonstiff; 1: stiff; 2: stiff, jac supplied */
  long    iDSFlag;        /* lsodes return flag */
  long    liw;            /* length of iwork array */
  long    lrw;            /* length of rwork array */
  PLONG   iwork;          /* working array pointer */
  PDOUBLE rwork;          /* working array pointer */

  /* Cvodes specifics */
  int    maxsteps;        /* Maximum number of internal steps before t_out */
  int    maxnef;          /* Maximum number of error test failures */
  int    maxcor;          /* Maximum number of nonlinear iterations */
  int    maxncf;          /* Maximum number of convergence failures */
  double nlscoef;         /* Coefficient in the nonlinear convergence test */
  
  /* Euler specifics */
  double  dTStep;         /* time step for Euler */

} INTSPEC, *PINTSPEC; /* tagINTSPEC */


/* Print Record: for info from a Print() statement */

typedef struct tagPRINTREC {
  PSTR szOutputName;
  HVAR hvar;
  long cTimes;
  PDOUBLE pdTimes;
} PRINTREC, *PPRINTREC; /* tagPRINTREC */


/* Data record: for info from a Data() statement */

typedef struct tagDATAREC {
  PSTR szDataName;
  HVAR hvar;
  long cData;
  PDOUBLE pdData;
} DATAREC, *PDATAREC; /* tagDATAREC */


/* Output specification */

typedef struct tagOUTSPEC {
  int     nOutputs;           /* Number of outputs statements */
  PLIST   plistPrintRecs;     /* List of records from Print()'s */
  PSTR    *pszOutputNames;    /* Array of output names */
  HVAR    *phvar_out;         /* Array of handles to outputs */

  int     nData;              /* Number of data statements */
  PLIST   plistDataRecs;      /* List of records from Data()'s */
  PSTR    *pszDataNames;      /* Array of output names */
  HVAR    *phvar_dat;         /* Array of handles to outputs */

  /* The lists are converted into the following */

  PINT    pcOutputTimes;    /* Count of output times for each output */
  PINT    piCurrentOut;     /* Index to current output for each output */
  PDOUBLE *prgdOutputTimes; /* Array of output times for each output */
  PDOUBLE *prgdOutputVals;  /* Array of output values for each output */

  int     cDistinctTimes;   /* Count of distinct output times for all
                               outputs */
  PDOUBLE rgdDistinctTimes; /* Array of distinct output times  for all
                               outputs */

  PINT    pcData;           /* Count of values for each data */
  PDOUBLE *prgdDataVals;    /* Array of values for each data */

} OUTSPEC, *POUTSPEC; /* tagOUTSPEC */


/* Monte Carlo Variation for one parameter */

typedef struct tagMCVAR {
  PSTR    pszName;          /* Model variable name */
  HVAR    hvar;             /* Handle to the model variable to be modified */
  double  dVal;             /* Current value */
  PDOUBLE pdVal;            /* Pointer to value */
  double  dVal_mean;        /* Sampled mean */
  double  dVal_var;         /* Sampled variance */

  int     iDepth;           /* Level (to distinguish vars with same hvar) */
  int     iType;            /* One of MCV_ distribution types */

  HVAR    hParm[4];         /* Handles to model vars for 4 distrib. params */
  double  dParm[4];         /* Values of fixed distribution parameters */
  PDOUBLE pdParm[4];        /* Pointers to symbolic distribution parameters */
  int     iParmType[4];     /* distrib. parameter types (const., param., 
                               pred., or data) */

  struct  tagMCVAR *pMCVParent[4]; /* Pointers to parents of this var (vars on
                                      which this var depends) */
  PLIST   plistDependents;  /* List of MCvars depending directly on this one */
  long    nDependents;
  struct  tagMCVAR **rgpDependents;

  BOOL    bExptIsDep;       /* True if experiment is dependent on this var */
  BOOL    bIsFixed;         /* True if var is fixed */
  BOOL    bGibbs;           /* True if its conditional distrib. is known */
  long    lJumps;           /* Number of MH jumps for this param */
  long    lCount;           /* Number of data values eventually attached */
  double  dKernelSD;        /* MCMC jumping kernel SD */
  double  dMaxKernelSD;     /* Maximum value of jumping kernel SD */

  PDOUBLE pdSum;            /* Running sum of sampled values */
  PDOUBLE pdSumSq;          /* Running sum of squared deviates from mean */

} MCVAR, *PMCVAR; /* tagMCVAR */


typedef struct tagGIBBSDATA {
  long    nMaxIter;          /* Number of iterations */
  long    nSimTypeFlag;      /* Number of iterations before vector sampling */
  long    nPrintFreq;        /* requests output every nPrintFreq iterations */
  long    nPrintIter;        /* Number of final iterations to print */
  long    nMaxPerkSetIter;   /* Maximum of iterations to set the perk scale */

  PSTR    szGout;            /* Filename for output */
  PFILE   pfileOut;          /* File pointer for output */

  PSTR    szGrestart;        /* Filename for restart parameter vectors */
  PFILE   pfileRestart;      /* File pointer for restart */

  PSTR    szGdata;           /* Filename for input data */

  PFILE   pfilePerks;        /* File pointer for perks scale */

  int     nPerks;            /* Number of perks for tempered MCMC */
  PDOUBLE rgdPerks;          /* Array of inverse temperatures (perks) */
  PLONG   rglTransAttempts; 
  PLONG   rglTransAccepts;
  int     indexT;            /* Index of current perk */
  PDOUBLE rgdlnPi;           /* Pseudo-priors (one per perk) */
  PLONG   rglCount;          /* Count of samples at given perks */
  double  dCZero;            /* Robbins-Munro updating rate parameter */
  double  dNZero;            /* Robbins-Munro updating decay parameter */
  int     startT;            /* Index of actual starting perk */
  int     endT;              /* Index of actual ending perk */

} GIBBSDATA, *PGIBBSDATA; /* tagGIBBSDATA */


/* Specification for Monte Carlo type experiment */
enum {forward, backward};

typedef struct tagMONTECARLO {
  long nRuns;               /* Number of Monte Carlo runs */
  long lRun;                /* Number of current Run */

  PSTR  szMCOutfilename;    /* File name for Monte Carlo output */
  PFILE pfileMCOut;         /* File for Monte Carlo output */

  PSTR  szSetPointsFilename;/* File name for set points */
  PFILE pfileSetPoints;     /* File of set points */

  PLIST plistMCVars;        /* List of MCVAR record, variation specs */

  /* The list is converted to the following */
  long   nParms;            /* Count of parameters */
  double *rgdParms;         /* The actually used parameter vector */
  HVAR   *rghvar;           /* Array of handles to the parameters */
  MCVAR  **rgpMCVar;        /* A priori distributions for each */

  long   nSetParms;         /* Count of setpoint parameters */

  int    style;             /* either forward or backward for optimal design */

} MONTECARLO, *PMONTECARLO; /* tagMONTECARLO */


/* Record of info about the model */

typedef struct tagMODELINFO {
  long      nStates;
  long      nModelVars;

  HVAR      *pStateHvar;	/* hvars of state variables */

  PDOUBLE   pdModelVars;

} MODELINFO, *PMODELINFO; /* tagMODELINFO */


/* Record of information for one experiment.
   An experiment specifies a set of experimental
   conditions, parameter settings, input levels, etc.
 */

typedef struct tagEXPERIMENT {
  int iExp;                 /* Number of this experiment */

  double dT0;               /* Initial time */
  HANDLE hT0;               /* Handle to initial time */

  double dTfinal;
  double dTime;             /* Current Time */

  PMODELINFO pmodelinfo;    /* Pointer to the model information */
  PLIST plistParmMods;      /* List of parameter mods (elt = PVARMOD) */

  INTSPEC is;               /* Integrator spec, this experiment */
  OUTSPEC os;               /* Output spec, this experiment */

  double dLnLike;           /* Log-likelihood */
  double dLnLikeSave;

} EXPERIMENT, *PEXPERIMENT; /* tagEXPERIMENT */


/* Information for each instance of a level */

typedef struct tagLEVEL {
  int    iDepth;                           /* Depth of this level */
  int    iSequence;                        /* Instance # of this level */
  int    iInstances;                       /* # of instances of next level */
  struct tagLEVEL *pLevels[MAX_INSTANCES]; /* Pointers to sublevels */

  PLIST   plistVars;     /* Vars, other than those in Distrib */
  long    nFixedVars;    /* Count of fixed parameters */
  PVARMOD *rgpFixedVars; /* Array of var values (from plistVars) */

  PLIST   plistMCVars; /* List of MCVAR records, variation specs */
  long    nMCVars;     /* Count of parameters */
  PMCVAR *rgpMCVars;   /* Array of MCVAR records (from plistMCVars) */

  PLIST   plistLikes;  /* List of MCVAR records, likelihood specs */
  long    nLikes;      /* Count of likelihoods */
  PMCVAR *rgpLikes;    /* Array of MCVAR records (from plistLikes) */

  PEXPERIMENT pexpt;   /* Ptr to expt struct, NULL if not expt
                          EXPERIMENT is used for compatibility */ 

} LEVEL, *PLEVEL; /* tagLEVEL */


/* Defines an analysis for an input file */

typedef struct tagANALYSIS {

  int  rank;                /* Parallel processor ID */
  int  size;                /* Number of parallel processors used */

  BOOL bDependents;         /* Debug flag for printing dependents to stderr */
  BOOL bOutputIter;         /* Flag for printing iteration numbers */
  int  nOutputFreq;         /* Flag for frequency of iteration printing */
  BOOL bPrintConvergence;   /* Flag for MCMC convergence printing */

  int  iType;               /* Type of analysis. One of AT_ types */

  WORD      wContext;       /* Context flag used during input processing */
  double    dSeed;          /* Random seed used for all analyses */

  MODELINFO modelinfo;      /* The model we are using */

  int iDepth;               /* Depth of levels */
  int iCurrentDepth;
  int iInstances;           /* Number of instances of level 1 */
  int iExpts;               /* Total number of experiments at all levels */

  PLEVEL pLevels[MAX_INSTANCES];    /* Pointer to level 1 structures */
  PLEVEL pCurrentLevel[MAX_LEVELS]; /* Pointers to currently nested structs */
  int iInstance[MAX_LEVELS];        /* Sequence of instances, e.g., toplevel 1,
                                       subject 2, experiment 3 */

  EXPERIMENT  expGlobal;            /* Global experiment settings */

  PSTR  szOutfilename;              /* Name of file for regular output */
  PFILE pfileOut;                   /* Pointer to file */
  BOOL  bCommandLineSpec;           /* Output file specified on command line */
  BOOL  bAllocatedFileName;         /* Output file name should be freed */

  PEXPERIMENT rgpExps[MAX_INSTANCES];  /* List of pointer to experiments */
  PEXPERIMENT pexpCurrent;             /* Experiment being currently defined */

  PLIST plistVars;      /* Global variables to set */

  MONTECARLO    mc;     /* Monte Carlo specification data */
  GIBBSDATA     gd;     /* MCMC specification data */

} ANALYSIS, *PANALYSIS; /* tagANALYSIS */


/* ----------------------------------------------------------------------------
   Globals/Externals
*/

extern PSTRLEX vrgszlexArgs[];


/* ----------------------------------------------------------------------------
   Prototypes
*/

void AnnounceProgram(int iRank);
void CorrectInputToTransition(PEXPERIMENT, PDOUBLE);
int  DoOneExperiment(PEXPERIMENT pexp);
void DoAnalysis(PANALYSIS panal);
void DoMonteCarlo(PANALYSIS panal);
void DoNormal(PANALYSIS panal);
int  DoOneMCExp(PANALYSIS panal, PEXPERIMENT pexp);
int  DoOneNormalExp(PANALYSIS panal, PEXPERIMENT pexp);
int  Euler(long neq, double *y, double *t, double tout, double dTStep);
void FreeVarMod(PVOID pData);
void GetCmdLineArgs(int cArg, char *const *rgszArg, PSTR *pszFileIn,
                    PSTR *pszFileOut, PANALYSIS panal);
void GetOutputFlagOption(PANALYSIS panal, char *optarg);
int  MCVarListToArray(PVOID pv_pMCVar, PVOID pv_Null);
int  ModifyOneParm(PVOID pData, PVOID pNullInfo);
void ModifyParms(PLIST plistParmMods);
void PrepAnalysis(PANALYSIS panal);
void PromptFilenames(PSTR *pszFileIn, PSTR *pszFileOut);
void WriteArray(FILE *pfile, long cElems, double *rg);
void WriteArrayLog(FILE *pfile, long cElems, double *rg);

#define SIM_H_DEFINED
#endif  /* SIM_H_DEFINED */

/* End */

