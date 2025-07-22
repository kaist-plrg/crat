/* random.h

   Copyright (c) 1992-2017 Free Software Foundation, Inc.

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

   Header for random number generator.  See random.c for extensive
   documentation.

   Gives prototypes for random functions.
*/

#ifndef RANDOM_H_DEFINED

#include <math.h>

#include "config.h"

/* ----------------------------------------------------------------------------
   Constants  */

#define SEED_MIN     1.0
#define SEED_MAX     2147483646.0

#ifdef HAVE_LIBGSL
#define SEED_DEFAULT 0
#else  /* non-gsl version */
#define SEED_DEFAULT 314159265.3589793
#endif


#define EXP1         2.7182818284590452353602875
#define INV_SQRT_2PI 0.398942280401432702863
#define LOG_SQRT_2PI 0.918938533204672669541
#define PI           3.1415926535897932384626433
#define SQRT_2       1.414213562373095145475
#define SQRT_2PI     2.506628274631000241612
#define TWO_SQRTEXP1 3.297442541400256388329


/* ----------------------------------------------------------------------------
   Prototypes  */

/* Initialize the random generators, mandatory */
void InitRandom (int rank, double dSeed, int bWarmUp);

/* Fundamental random generator */
double Randoms (void);

/* Several types of random variates */
double BetaRandom (double alpha, double beta, double a, double b);
double BinomialBetaRandom (double Expectation, double alpha, double beta);
double BinomialRandom (double p, long n);
double CauchyRandom (double dScale);
double Chi2Random (double dof);
double ExpRandom (double beta);
double InvGGammaRandom (double alpha, double beta);
double TruncInvGGammaRandom (double alpha, double beta, double a, double b);
double GammaRandom (double alpha);
double GGammaRandom (double alpha, double beta);
double LogNormalRandom (double dGeoMean, double dGeoSD);
double GenLogNormalRandom (double dMean, double dSDNorm, double dSDLogNorm);
double StudentTRandom (double dof, double dMean, double dSD);
double LogUniformRandom (double a, double b);
long   NegativeBinomialRandom (double r, double p);
double NormalRandom (double dMean, double dSD);
double PiecewiseRandom (double min, double a, double b, double max);
double PiecewiseVariate (long cDim, double rg_x[], double rg_pdf[],
                         double rg_Cdf[], int iOrder, double *pVal_pdf);
long   PoissonRandom (double mu);
double TruncLogNormalRandom (double dGeoMean, double dGeoSD,
                             double a, double b);
double TruncNormalRandom (double dMean, double dSD, double a, double b);
double UniformRandom (double a, double b);
void   Multinomial (long n, int dim, double *p, double *x);
void   WishartRandom (long n, long p, double *t, double *w, double *work);


/* ----------------------------------------------------------------------------
   Utility functions */

BOOL   and (BOOL A, BOOL B);
double CDFNormal (double z);
double DFNormal (double x, double mu, double sd);

#ifndef HAVE_ERFC
double erfc (double x);
#endif

double GetSeed (void);
double InterpolateX (double rgX[], double rgY[], long lLower, double dY);
double lnDFNormal (double x, double mu, double sd);
double lnGamma (double x);
double lnDFBeta (double x, double alpha, double beta, double min, double max);
void   CalcCumulative (long cDim, double *rg_x, double *rg_pdf,
                       double *rg_Cdf, int  iOrder);
void   SetSeed (double dSeed);
 
#define RANDOM_H_DEFINED
#endif

/* End */

