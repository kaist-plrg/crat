/* random.c

   Copyright (c) 1992-2018 Free Software Foundation, Inc.

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


 * Random number generator module: Two versions are provided. One uses the
   GNU Scientific Library (GSL). The other is independent of GSL.

   The GSL version provides:
   Randoms(), which in fact call gsl_rng_uniform()
   InitRandom(rank, seed, f), must be called to initializes the generator
   SetSeed(seed), which sets the seed of Randoms()
   GetSeed(seed), not implemented in fact and exits with an error message

   The independent (non-GSL) code provides two basic uniform random number
   generators and associated functions:
   Randoms(), yielding uniform random variates between 0 and 1
   InitRandom(rank, seed, f), which initializes the seed
   SetSeed(seed), which sets the seed of Randoms()
   GetSeed(seed), which gets the current seed

 * Other types of random variates generators are available. They call
   Randoms() if they require a uniform variate:

   BetaRandom(alpha, beta, a, b)      -- Beta(alpha, beta) over [a, b]
   BinomialBetaRandom(E, a, b)        -- BetaBinomial(n = E + E * a / b)
   BinomialRandom(p, n)               -- Binomial of n trials, P(each) = p
   CauchyRandom(s)                    -- Cauchy, with scale s
   Chi2Random(dof)                    -- Chi-squared w/dof degrees of freedom
   ExpRandom(beta)                    -- Exponential of inverse scale beta
   GammaRandom(alpha)                 -- Gamma variate
   GenLogNormalRandom(m, sn, sln)     -- Generalized log transformation of the
                                         two-component error model
   GGammaRandom(alpha, beta)          -- General gamma variate
   InvGGammaRandom(alpha, beta)       -- General inverse gamma variate
   LogNormalRandom(m, s)              -- exp(Normal)
   LogUniformRandom(a, b)             -- LogUniform over [a, b]
   Multinomial(...)                   -- Multinomial variates
   NegativeBinomialRandom(r, p)       -- Negative Binomial variates
   NormalRandom(m, s)                 -- General Normal
   PiecewiseRandom(min, a, b, max)             -- Draws from a Mayan pyramid !
   PiecewiseVariate(n, x[], p[], Cdf[], o, p)  -- Draws from a tabulated PDF
   PoissonRandom(mu)                  -- Poisson with rate mu
   StudentTRandom(dof, m, s)          -- Student-t with dof degrees of freedom
                                         location m and scale s
   TruncInvGGammaRandom(alpha, beta, a, b)     -- Truncated general inv-gamma
   TruncLogNormalRandom(m, s, a, b)   -- Truncated log normal
   TruncNormalRandom(m, s, a, b)      -- Truncated general normal
   UniformRandom(a, b)                -- Uniform over [a, b]
   WishartRandom(...)                 -- Wishart(matrix) variates

 * And utility functions:

   and()                                     -- Logical "and" function
   CDFNormal(z)                              -- Integral of the normal at z
   CalcCumulative(n, x[], p[], Cdf[], o)     -- Constructs a CDF given a PDF
   DFNormal(x, mu, sd)                       -- Normal density
   erfc(x)                                   -- Complementary error function
   leq()                                     -- Logical lower or equal function
   lnDFNormal(x, mu, sd)                     -- Log of the normal density
   lnGamma(x)                                -- Natural log of gamma function
   lnDFBeta(x, alpha, beta, min, max)        -- Log of beta density
   piecewise(...)                            -- Piecewise function

*/

#include <float.h>
#include <assert.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "hungtype.h"
#include "lex.h"
#include "lexerr.h"
#include "random.h"


/* ----------------------------------------------------------------------------
   Globals/Externals
*/

static BOOL vbSwitchGauss = FALSE; /* Flag to reset switchG in NormalRandom */


/* ----------------------------------------------------------------------------
   We have two versions of Randoms() depending whether we use gsl or not
*/

#ifdef HAVE_LIBGSL

#include <gsl/gsl_rng.h>

/* ----------------------------------------------------------------------------
   GSL version, global definitions, private
*/

static const gsl_rng_type * genType;
static gsl_rng * rGenerator;

/* ----------------------------------------------------------------------------
   GetSeed

   Not available if GSL is used. Issue an error message.
*/
double GetSeed (void)
{
  printf ("Error: the GetSeed() function is not available if GSL is used "
          "- Exiting\n\n");
  exit (0);
}


/* ----------------------------------------------------------------------------
   SetSeed

   Sets vRandRec.seed to given dSeed. Note that the seed is a long int.
*/
void SetSeed (double dSeed)
{

  gsl_rng_set(rGenerator, (unsigned long int) dSeed);
  vbSwitchGauss = FALSE; /* Force a reset of the normal variate sampler */

} /* SetSeed */


/* ----------------------------------------------------------------------------
   Randoms, gsl version
*/
double Randoms (void)
{

  return(gsl_rng_uniform(rGenerator));

} /* Randoms, gsl version */


/* ----------------------------------------------------------------------------
   InitRandom, gsl version

   Selects a generator and sets the gsl seed to given dSeed, silently
   corrects an invalid dSeed.
*/
void InitRandom (int rank, double dSeed, int rdm_gen_name)
{
  static int bInit = 0;

  if (!bInit) { /* initialize */

    /* set the type of random generator, in fact we should use an enum */
    switch (rdm_gen_name) {
    case TRUE:
      genType = gsl_rng_default; /* mt19937 */
      break;
    case FALSE:
      genType = gsl_rng_taus2;
      break;
    default:
      genType = gsl_rng_default;
    }

    /* create an instance of a generator of type genType */
    rGenerator = gsl_rng_alloc(genType);

    /* set the seed */
    gsl_rng_set(rGenerator, (unsigned long int) dSeed);

    vbSwitchGauss = FALSE; /* Force a reset of the normal variate sampler */

    /* next calls to InitRandoms will do nothing */
    bInit = 1;
  }

} /* InitRandom, gsl version */


#else  /* non-gsl version */

/* ----------------------------------------------------------------------------
   Non-GSL version, global definitions, private
*/

typedef struct tagRANDREC {
  double seed;
} RANDREC, *PRANDREC;


static BOOL vbNoSeed = TRUE;       /* Flag to prevent use without seed */
static BOOL vbNotInitd = TRUE;     /* Flag to prevent use without initing */
static RANDREC vRandRec;           /* Global random information shared by */
                                   /* all random number functions */

/* ----------------------------------------------------------------------------
   GetSeed

   Returns the current value of vRandRec.seed.
*/
double GetSeed (void)
{
  return (vRandRec.seed);
}


/* ----------------------------------------------------------------------------
   SetSeed

   Sets vRandRec.seed to given dSeed, silently corrects an invalid dSeed.
*/
void SetSeed (double dSeed)
{
  int bCorrected = 0;

  if (dSeed == 0.0) {
    dSeed = SEED_DEFAULT;
    bCorrected++;
  }

  if (dSeed < 0)
    dSeed = -dSeed; /* Don't announce this correction */

  if (dSeed < SEED_MIN) {
    dSeed = SEED_MIN + (dSeed/SEED_MIN) / (SEED_MAX-SEED_MIN);
    bCorrected++;
  }

  if (dSeed > SEED_MAX) {
    dSeed = SEED_MIN + (SEED_MAX/dSeed) / (SEED_MAX-SEED_MIN);
    bCorrected++;
  }

  assert ((/* Invalid Seed */ dSeed >= SEED_MIN && dSeed <= SEED_MAX));

  /* Assign valid seed */

  if (bCorrected)
    printf ("SetSeed():  corrected out of range random number seed\n"
            "Seed must lie in the range [%g, %g]\n"
            "New seed --> %g\n", SEED_MIN, SEED_MAX, dSeed);

  vRandRec.seed = dSeed;
  vbNoSeed = FALSE;      /* Flag that seed has been set */
  vbSwitchGauss = FALSE; /* Force a reset of the normal variate sampler */

} /* SetSeed */


/* ----------------------------------------------------------------------------
   InitRandom

   initializes the random generator with the given seed.
   If an invalid seed is given, SetSeed() silently corrects it.

   If the boolean bWarmUp is non-zero, the random number generator is
   "warmed up" by running it a number of times.
   Also, a flag used by the Normal() routine is initialized.
*/
void InitRandom (int rank, double dSeed, int bWarmUp)
{
  long i;

  /* Prevent nuking user's seed if not initd */
  if (vbNoSeed || dSeed != SEED_DEFAULT)
    SetSeed(dSeed);

  if (bWarmUp) {
    /* Warm up generator */
    for (i = 0; i < 50; i++) (void) Randoms();

    vbNotInitd = FALSE; /* Flag as initialized */
  }

} /* InitRandom, non-gsl version */


/* ----------------------------------------------------------------------------
   Randoms

   An alternative random number generator, so you don't have to use
   the (probably not so good) system supplied standard C version.

   Randoms() returns random numbers between 0 and 1. The minimum
   returned value is 1/m and the maximum 1 - 1/m. The generator can
   be initialized with InitRandom(). If not, it auto-initializes its seed.

   This generator should be correct on any system for which the
   representattion of reals uses at least a 32-bit mantissa, including
   the sign bit.

   From PARK SK, MILLER KW: Random number generators: good ones are
   hard to find.  Commun. ACM 1988; 31: 1192. (Version Real 2).
*/
double Randoms (void)
{
#define a  16807.0
#define m  2147483647.0
#define q  127773.0   /* m Div a */
#define r  2836.0     /* m Mod a */

  double hi, test;

  if (vbNoSeed)
    SetSeed (SEED_DEFAULT);

  hi = (long)(vRandRec.seed / q);
  test = a * (vRandRec.seed - q * hi) - r * hi;

  if (test > 0.0)
    vRandRec.seed = test;
  else
    vRandRec.seed = test + m;

  return (vRandRec.seed / m);

#undef a
#undef m
#undef q
#undef r

} /* Randoms, non-gsl version */

#endif

/* End of the non-gsl random uniform generators */


/* ----------------------------------------------------------------------------
   BetaRandom

   returns a variate that is Beta distributed on the interval [a,b]
   with shape parameters alpha and beta.

   The Beta function has two shaping parameters, alpha and beta.
   Setting these parameters to 1.5 and 1.5 yields a normal-like
   distribution, but without tails. If alpha and beta are equal to
   1 it is a uniform distribution.

   If alpha and beta are less than 1, use a rejection algorithm;
   Otherwise use the fact that if x is distributed Gamma(alpha) and y
   Gamma(beta) then x/(x+y) is Beta(alpha, beta).

   For the rejection algorithm first a Beta variate is found over the
   interval [0, 1] with not the most efficient algorithm.  This is then
   scaled at the end to desired range.

   It may be tempting to re-use the second number drawn as the first
   random number of the next iteration, and simply draw one more.
   *** Don't do it. You will produce an incorrect distribution. You
   must draw two new numbers for the rejection sampling to be correct.

   References:
   - Ripley, Stochastic Simulations, John Wiley and Sons, 1987, p 90.
   - J.H.Maindonald, Statistical Computation, John Wiley and Sons,
     1984, p 370.
*/
double BetaRandom (double alpha, double beta, double a, double b)
{
  double u1, u2, w;

  if (b <= a || alpha <= 0 || beta <= 0) {
    printf ("Error: bad shape or range for a beta variate - Exiting\n\n");
    exit (0);
  }

  if ((alpha < 1) && (beta < 1))
    /* use rejection */
    do {
      u1 = Randoms(); /* Draw two numbers */
      u2 = Randoms();

      u1 = pow(u1, 1/alpha); /* alpha and beta are > 0 */
      u2 = pow(u2, 1/beta);

      w = u1 + u2;

    } while (w > 1.0);

  else {
    /* use relation to Gamma */
    u1 = GammaRandom(alpha);
    u2 = GammaRandom(beta);
    w  = u1 + u2;
  }

  return (a + (u1/w) * (b - a)); /* Scale to interval [a, b] */

} /* BetaRandom */


/* ----------------------------------------------------------------------------
   BinomialBetaRandom

   Return as a double floating-point number an integer value that is a random
   deviate drawn from a binomial distribution of n trials each of
   probability p, p being beta distributed with parameters alpha and beta.
   I use the expectation in input. The classical N is equal to
   E + E * beta / alpha.
   See Bernardo & Smith "Bayesian Theory"
*/
double BinomialBetaRandom (double Expectation, double alpha, double beta)
{
  double dTmp = Expectation + Expectation * beta / alpha;

  if (dTmp < LONG_MAX)
    /* parameters will be checked in BinomialRandom */
    return BinomialRandom (BetaRandom (alpha, beta, 0, 1), (long) dTmp);
  else {
      printf("BinomialBetaRandom: N (= %g) too large - Exiting...", dTmp);
      exit (0);
  }

} /* BinomialBetaRandom */


/* ----------------------------------------------------------------------------
   BinomialRandom

   Return as a double floating-point number an integer value that is a random
   deviate drawn from a binomial distribution of n trials each of
   probability p, using Randoms() as a source of uniform random deviates.
   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al.
*/
double BinomialRandom (double p, long N)
{
  long j;
  static long iOldN = -1;
  double dAngle, dDeviate, dMean, dPtemp, dSqrt, dTangent, dTemp1, dTemp2;
  static double dLnFactN, dPold = -1, dLnP, dQ, dLnQ;

  if (p < 0 || p > 1 || N < 0) {
    printf ("Error: parameters out of bounds for a binomial variate "
            "- Exiting\n\n");
    exit (0);
  }

  dPtemp = ( p <= 0.5 ? p : 1 - p);
  dMean = N * dPtemp;  /* mean of the deviate to be produced. */

  /* Use the direct method if N is not too large.
     This can require up to 25 calls to random */

  if (N < 25) {
    dDeviate = 0;
    for (j = 0; j < N; j++)
      if (Randoms() < dPtemp)
        dDeviate = dDeviate + 1;
  }
  else
    if (dMean < 1) {
      /* if less than one event is expected out of 25 or more trials,then the
         distribution is quite accurately Poisson. Use direct method. */
      dTemp1 = exp(-dMean);
      dTemp2 = 1.0;
      for (j = 0; j <= N; j++) {
        dTemp2 = dTemp2 * Randoms();
        if (dTemp2 < dTemp1) break;
      }

      dDeviate = (j <= N ? j : N);
    }
    else { /* Use rejection */

      if (N != iOldN) {
        /* if N has changed or it's the first call, initialize */
        dLnFactN = lnGamma((double) N + 1);
        iOldN = N;
      }

      if (dPtemp != dPold) {
        /* if dPtemp has changed or it's the first call, initialize. */
        dPold = dPtemp;
        dQ = 1 - dPtemp;
        dLnP = log(dPtemp);
        dLnQ = log(dQ);
      } /* if */

      dSqrt = sqrt(2 * dMean * dQ);

      /* Rejection method with a Lorentzian comparison function. */

      do {
        do {
          dAngle = PI * Randoms();
          dTangent = tan(dAngle);
          dTemp1 = dSqrt * dTangent + dMean;
        } while (dTemp1 < 0 || dTemp1 >= (N + 1)); /* Reject */

        dTemp1 = floor(dTemp1); /* discrete distribution */

        dTemp2 = 1.2 * dSqrt * (1 + dTangent * dTangent) *
                 exp(dLnFactN - lnGamma(dTemp1 + 1) -
                     lnGamma(N - dTemp1 + 1) +
                     dTemp1 * dLnP + (N - dTemp1) * dLnQ);

      } while (Randoms() > dTemp2);

      /* Reject on average about 1.5 time per deviate */

      dDeviate = dTemp1;

    } /* else */  /* end of rejection */

  if (dPtemp != p)
    dDeviate = N - dDeviate; /* undo the symmetry tranformation */

  return (dDeviate);

} /* BinomialRandom */


/* ----------------------------------------------------------------------------
   CauchyRandom

   Returns a random variate from Cauchy's distribution (generated by dividing a
   standard Normal by a Chi-square with 1 degree of freedom.
*/
double CauchyRandom (double dScale)
{
  double z, x;

  z = NormalRandom(0, dScale);
  x = GGammaRandom (0.5, 0.5); /* Chi-2, see Chi2Random */

  return (z / sqrt(x));

} /* CauchyRandom */


/* ----------------------------------------------------------------------------
   Chi2Random

   returns a chi-squared random variate, which is a gamma(dof/2, 1/2).
*/
double Chi2Random (double dof)
{

  return (GGammaRandom (dof / 2.0, 0.5));

} /* Chi2Random */


/* ----------------------------------------------------------------------------
   ExpRandom

   returns an exponential variate with inverse scale beta

   Algorithm 3.2 from Ripley "Stochastic Simulations" Wiley 1987, p. 55.
*/
double ExpRandom (double beta)
{

  if (beta <= 0) {
    printf ("Error: negative or null inverse scale for an exponential variate "
            "- Exiting\n\n");
    exit (0);
  }

  return -log(Randoms()) / beta;

} /* ExpRandom */


/* ----------------------------------------------------------------------------
   GammaRandom

   returns a gamma distributed random variate with shape parameter
   alpha.

   If alpha < 1 uses algorithm 3.19 of Ripley; if alpha > 1 uses
   algorithm 3.20 of Ripley; if alpha is 1 uses returns an
   ExpRandom variate.

   Reference:
   - Ripley, Stochastic Simulations, John Wiley and Sons, 1987, pp 88-90.
*/
double GammaRandom (double alpha)
{
#define E 2.718281828459
  static double aprev = 0.0, c1, c2, c3, c4, c5;
  double b, u1, u2, w, x;

  if (alpha <= 0) {
    printf ("Error: negative or null shape parameter for a gamma variate "
            "- Exiting\n\n");
    exit (0);
  }
  else if (alpha < 1) {

    b = (alpha + E) / E;

    do {
      u1 = b * Randoms();
      if (u1 <= 1.0) {
        x = pow(u1, 1./alpha);
        /* problem: if alpha is too small, x will be about zero and that's
           bad, particularly for inverse-gamma variates.
           Fixed by blocking zeros - FB 5/11/1997 */
        if ((x > DBL_MIN) && (x <= -log(Randoms())))
          return(x);
      }
      else {
        x = -log((b - u1) / alpha);
        if (pow(x, alpha - 1) >= Randoms()) return(x);
      }
    } while (1 == 1);

  } /* end if alpha < 1 */

  else {
    if (alpha > 1) {

      if (alpha != aprev) {
        /* initialize */
        aprev = alpha;
        c1 = alpha - 1;
        b = 1.0 / c1;
        c2 = b * (alpha - (1 / (6.0 * alpha)));
        c3 = 2 * b;
        c4 = c3 + 2.0;
        if (alpha > 2.5)
          c5 = 1.0 / sqrt(alpha);
      }

      do {
        do {
          u1 = Randoms();
          u2 = Randoms();
          if (alpha > 2.5)
            u1 = u2 + c5 * (1 - 1.86 * u1);
        } while ((u1 >= 1) || ( u1 <= 0));

        w = c2 * u2 / u1;
        if (((c3 * u1 + w + 1 / w) <= c4) ||
            ((c3 * log(u1) - log(w) + w) < 1))
          return(c1 * w);
      } while (1 == 1);

    }
    else
      return ExpRandom(1.0);
  }

  #undef E

} /* GammaRandom */


/* ----------------------------------------------------------------------------
   GenLogNormalRandom

   Returns a variate drawn from the generalized lognormal distribution,
   which is an approximation of the two-component error model of Rocke
   and Lorenzato 1995. At low values of the mean, it is approximately
   normal and at high values of the mean, it is approximately lognormal

   dMean and dSDNorn are in natural space.
   Note that here SDLogNorm is sigma in log space; this convention
   is different from that for the LogNormal.
*/
double GenLogNormalRandom (double dMean, double dSDNorm,
			   double dSDLogNorm)
{
  double dmuz, dSLogNorm, dLambda, dz;

  if (dMean < 0) { /* "True value must be >= 0 */
    char str[10];
    sprintf(str, "%5.2e", dMean);
    ReportRunTimeError(NULL, RE_BADLOGNORMALMEAN | RE_FATAL,
                       "", str, "GenLogNormalRandom");
  }
  else
    if (dSDLogNorm <= 0) {
      char str[10];
      sprintf(str, "%5.2e", dSDLogNorm);
      ReportRunTimeError(NULL, RE_BADLOGNORMALSD | RE_FATAL,
                         "", str, "GenLogNormalRandom");
    }

  /* Relative Standard Deviation of Lognormal */
  dSLogNorm = sqrt(exp(pow(dSDLogNorm,2)) * (exp(pow(dSDLogNorm,2)) - 1));
  /* Transformation parameter */
  dLambda = pow(dSDNorm/dSLogNorm,2);
  /* Transformation of mean */
  dmuz = log(dMean + sqrt(pow(dMean,2) + dLambda));
  /* Random variate of transformed variable */
  dz = NormalRandom(dmuz, dSLogNorm);

  return (exp(dz) - dLambda * exp(-dz))/2;  /* return untransformed variable */

} /* GenLogNormalRandom */


/* ----------------------------------------------------------------------------
   GGammaRandom

   Returns a gamma distributed random variate with shaping parameter
   alpha and inverse scale (rate) parameter beta (> 0).
*/
double GGammaRandom (double alpha, double beta)
{

  if (beta <= 0) {
    printf ("Error: negative or null inverse scale for a gamma variate "
            "- Exiting\n\n");
    exit (0);
  }
  return GammaRandom(alpha) / beta;

} /* GGammaRandom */


/* ----------------------------------------------------------------------------
   InvGGammaRandom

   Returns an inverse gamma distributed random variate with shape parameter
   alpha and scale parameter beta.
   This just gets a general gamma variate and returns its inverse
   See Gelman et al. "Bayesian Data Analysis"
*/
double InvGGammaRandom (double alpha, double beta)
{

  /* parameters will be checked in GGammaRandom */
  if (beta <= 0) {
    printf ("Error: negative or null scale for an inverse gamma variate "
            "- Exiting\n\n");
    exit (0);
  }

  return beta / GammaRandom(alpha);

} /* InvGGammaRandom */


/* ----------------------------------------------------------------------------
   LogNormalRandom

   returns a variate such that the log of the variate is normally
   distributed.
   Uses the geometric mean and geometric standard deviation.
*/
double LogNormalRandom (double dGeoMean, double dGeoSD)
{

  if (dGeoMean <= 0) {
    char str[10];
    sprintf(str, "%5.2e", dGeoMean);
    ReportRunTimeError(NULL, RE_BADLOGNORMALMEAN | RE_FATAL,
                       "", str, "LogNormalRandom");
  }
  else {
    if (dGeoSD < 1) {
      char str[10];
      sprintf(str, "%5.2e", dGeoSD);
      ReportRunTimeError(NULL, RE_BADLOGNORMALSD | RE_FATAL,
                         "", str, "LogNormalRandom");
    }
  }

  return exp(NormalRandom (log(dGeoMean), log(dGeoSD)));

} /* LogNormalRandom */


/* ----------------------------------------------------------------------------
   LogUniformRandom

   returns a variate that is log-uniformly distributed on the interval
   [a,b].
*/
double LogUniformRandom (double a, double b)
{

  if (b < a) {
    printf ("Error: bad range a for uniform variate - Exiting\n\n");
    exit (0);
  }

  return ( a * pow(b/a, Randoms()) );

} /* LogUniformRandom */


/* ----------------------------------------------------------------------------
   Multinomial

   Generates multinomial deviates.
   N is the number of trials,
   p the array of probabilities,
   dim the dimension of the array (number of possible events),
   x the array of event occurences which is returned.

   From Devroye "Non-Uniform Random Numbers...".
*/
void Multinomial (long n, int dim, double *p, double *x)
{
  int i;
  double sum, ptemp;

  sum = 1;

  for (i = 1; i <= dim; i++) {
    if (p[i]) {
      ptemp = p[i] / sum;
      x[i] = BinomialRandom (ptemp, n);
      n = n - (long)x[i];
      sum = sum - p[i];
    }
    else x[i] = 0.0;

  } /* for */

} /* Multinomial */


/* ----------------------------------------------------------------------------
   NegativeBinomialRandom

   Generates negative binomial deviates.
   r (> 0) is number of failures until experiments are stopped
   p (in [0,1]) is the success probability in each experiment (real)

   Adapted from Devroye "Non-Uniform Random Numbers..." page 543.
*/
long NegativeBinomialRandom (double r, double p)
{
  if (p < 0) {
    printf ("Error: parameter p negative for a negative binomial variate "
            "- Exiting\n\n");
    exit (0);
  }

  if (p >= 1) {
    printf ("Error: parameter p >= 1 for a negative binomial variate "
            "- Exiting\n\n");
    exit (0);
  }

  if (r < 0) {
    printf ("Error: parameter r negative for a negative binomial variate "
            "- Exiting\n\n");
    exit (0);
  }

  if (r == 0 || p == 0) return(0);

  return(PoissonRandom(GGammaRandom(r, (1 - p) / p)));

} /* NegativeBinomialRandom */


/* ----------------------------------------------------------------------------
   NormalRandom

   Returns a Normal random variate based on a unit variate,
   using a random generator as a source of uniform deviates.
   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al.
*/
double NormalRandom (double dMean, double dSD)
{
  double dRacine, dTemp1, dTemp2, dTemp3;
  static double memGauss;

  if (vbSwitchGauss) { /* used stored variate */
    vbSwitchGauss = FALSE;
    return (dMean + dSD * memGauss);
  }

  do {
    dTemp1 = 2 * Randoms() - 1;
    dTemp2 = 2 * Randoms() - 1;
    dRacine = dTemp1 * dTemp1 + dTemp2 * dTemp2;
  } while ((dRacine >= 1) || (dRacine == 0));

  dTemp3 = sqrt(-2 * log(dRacine) / dRacine);
  vbSwitchGauss = TRUE;
  memGauss = dTemp1 * dTemp3;
  return (dMean + dSD * (dTemp2 * dTemp3));

} /* NormalRandom */


/* ----------------------------------------------------------------------------
   PiecewiseRandom
                                                  __
   returns a variate that is distributed along a /  \ shaped distribution.
*/
double PiecewiseRandom (double min, double a, double b, double max)
{
  double dTemp;
  static double Grille[4];
  static double densite[4];
  static double densiteCum[4];
  double nvlle_densite;

  Grille[0] = min;
  Grille[1] = a;
  Grille[2] = b;
  Grille[3] = max;
  densite[0] = 0;
  densite[1] = 1/(max/2+b/2-a/2-min/2);
  densite[2] = 1/(max/2+b/2-a/2-min/2);
  densite[3] = 0;

  CalcCumulative (4, Grille, densite, densiteCum, 1);

  dTemp = PiecewiseVariate (4, Grille, densite, densiteCum, 1,
                            &nvlle_densite);
  return (dTemp);

} /* PiecewiseRandom */


/* ----------------------------------------------------------------------------
   PiecewiseVariate

   Returns a variate drawn by tabulated inversion from the
   cumulative, Cdf[], calculated to order iOrder
   (0 = piecewise-constant, etc.)

   Inputs: dimension of the table, x values, cdf values at x,
   pdf values at x, order of the interpolation.

   Returns the sampled variate as its value. If a pointer is given,
   the value of pdf[] at the sampled variate is returned in *pVal_pdf.

   Note: For piecewise-constant variates, the grid is corrected with
         CorrectPWConstantGrid() to center the intervals around
         sampled points.
*/
double PiecewiseVariate (long cDim, double rg_x[], double rg_pdf[],
                         double rg_Cdf[], int iOrder, double *pVal_pdf)
{
  double dPWVariate; /* the variate chosen */
  double dValPdf;    /* the value of the pdf at variate */
  double dUniform = UniformRandom(0, rg_Cdf[cDim - 1]);
  long   lUpper, lLower, lIndex;

  if (iOrder > 1) {
    printf ("CalcCumulative: Order %d not supported"
                     "-> using piecewise-linear\n", iOrder);
    iOrder = 1;
  }

  /* Find bounding Xs by a binary search of the ordered rg_x's
   */
  lUpper = cDim;
  lLower = 0;
  lIndex = 0;

  while (lUpper - lLower > 1) {
    lIndex = (lUpper + lLower)/2;

    if (dUniform > rg_Cdf[lIndex]) lLower = lIndex; /* Move to right half */
    else
      if (dUniform < rg_Cdf[lIndex])
        lUpper = lIndex; /* Move to left half */
      else lUpper = lLower = lIndex;
  }

  /* If we are exactly on a cumulative data point (unlikely),
     the value of the pdf is known and the variate is a grid point.
   */
  if (lUpper == lLower) {
    dValPdf = rg_pdf[lLower];
    dPWVariate = rg_x[lLower];
  }

  /* Otherwise we do the appropriate interpolation
   */
  else
    switch (iOrder) {

    /* Piecewise-constant pdf: the Cdf is piecewise-linear
     */
    case 0:
      dValPdf = rg_pdf[lLower];
      dPWVariate = InterpolateX (rg_x, rg_Cdf, lLower, dUniform);
      break;

    /* Piecewise-linear pdf: the Cdf is piecewise-quadratic
     */
    case 1: {

      if (rg_pdf[lLower] == rg_pdf[lUpper]) { /* A linear segment */
        dValPdf = rg_pdf[lLower];
        dPWVariate = InterpolateX (rg_x, rg_Cdf, lLower, dUniform);
      }

      else { /* Interpolate a quadratic */

        double a, b, c, dRadical;

        /* Find a, b, and c from the quadratic equation.
           a is guaranteed not zero from if() above. */

        a = (rg_pdf[lUpper] - rg_pdf[lLower]) /(rg_x[lUpper] - rg_x[lLower]);

        b = rg_pdf[lLower] - a*rg_x[lLower];

        c = rg_Cdf[lLower] - (a*rg_x[lLower]/2.0 + b) * rg_x[lLower];

        dRadical = sqrt(b*b - 2*a*(c - dUniform));

        dPWVariate = (-b + dRadical) / a;

        assert (dPWVariate >= rg_x[lLower] && dPWVariate <= rg_x[lUpper]);

        dValPdf = a * dPWVariate + b;

        if (a > 0)
          assert (dValPdf >= rg_pdf[lLower] && dValPdf <= rg_pdf[lUpper]);
        else
          assert (dValPdf <= rg_pdf[lLower] && dValPdf >= rg_pdf[lUpper]);

      } /* else */

    } /* case block */
    break;

    default:
      dValPdf = 0;
      dPWVariate = 0;
      assert(0);
      break;

  } /* switch */

  if (pVal_pdf)    *pVal_pdf = dValPdf; /* Return the value if requested */

  return dPWVariate;

} /* PiecewiseVariate */


/* ----------------------------------------------------------------------------
   PoissonRandom

   returns a Poisson random variate, with rate mu.

   If mu is less than 60, uses inversion; otherwise uses the rejection
   method of Atkinson (as presented by Ripley "Stochastic Simulations",
   Wiley 1987, p 79).
*/
long PoissonRandom(double mu)
{

  double u1, x, u2, lnfact, s, t;
  static double prev_mu = 0, c, beta, alpha, k;
  long n = 0;

  if (mu <= 0) {
    printf("Error: negative or null rate for a Poisson variate "
           "- Exiting\n\n");
    exit(0);
  }

  if (mu <= 60) {
    /* inversion */
    s = 1;
    t = 1;
    u1 = Randoms() * exp(mu);
    while(s < u1){
      n++;
      t = t * mu / n;
      s = s + t;
    }
  }
  else {
    /* rejection */
    if (mu != prev_mu) {
      c = 0.767 - 3.36 / mu;
      beta = PI / sqrt(3 * mu);
      alpha = beta * mu;
      k = log(c) - mu - log(beta);
    }

    do {
      do {
        u1 = Randoms();
        x = (alpha - log((1 - u1) / u1)) / beta;
      } while (x <= -0.5);

      n = (long)(x + 0.5);
      u2 = Randoms();

      /* calculate log n factorial using Stirling's formula */
      lnfact = 0.918938533 - n + (n + 0.5) * log(n);
    } while (alpha - beta * x + log(u2 / pow((1 + exp(alpha - beta * x)), 2))
             > k + n * log(mu) - lnfact);
  }

  return n;

} /* PoissonRandom */


/* ----------------------------------------------------------------------------
   StudentTRandom

   Returns a random variate from Student T distribution with dof
   degrees of freedom, location dMean, and scale dSD.
*/
double StudentTRandom (double dof, double dMean, double dSD)
{

  double z, x;

  if (dof <= 0) {
    printf ("Error: StudentTRandom: dof <= 0\n");
    exit (0);
  }
  z = NormalRandom(0,1);
  x = Chi2Random(dof);

  return (dMean + dSD*z*sqrt(dof/x));

} /* StudentTRandom */


/* ----------------------------------------------------------------------------
   TruncInvGGammaRandom

   returns a truncated general inverse gamme variate in the range [a, b].
*/
double TruncInvGGammaRandom (double alpha, double beta, double a, double b)
{
  double X = 0.0;
  int    iter = 0;

  if (a >= b)
    printf ("TruncLogNormalRandom: min >= max  [%g %g]\n", a, b);

  else do {
    if(++iter == 25) {
      printf("TruncInvGGammaRandom: problem with range: ");
      printf("min %g, max %g, alpha %g, beta %g\n", a, b, alpha, beta);
    }
    X = InvGGammaRandom(alpha, beta);
  }
  while (X < a || X > b);

  return X;

} /* TruncInvGGammaRandom */


/* ----------------------------------------------------------------------------
   TruncLogNormalRandom

   returns a truncated LogNormal variate in the range [a, b].
   All parameters are in natural space.
*/
double TruncLogNormalRandom (double dGeoMean, double dGeoSD,
                             double a, double b)
{
  
  return exp(TruncNormalRandom(log(dGeoMean), log(dGeoSD), log(a), log(b)));

} /* TruncLogNormalRandom */


/* ----------------------------------------------------------------------------
   TruncLogNormalRandom_old

   returns a truncated LogNormal variate in the range [a, b].
   All parameters are in natural space.
   Deprecated.
*/
double TruncLogNormalRandom_old (double dGeoMean, double dGeoSD,
                                 double a, double b)
{
  double X = 0.0;
  int    iter = 0;

  if (a >= b)
    printf ("TruncLogNormalRandom: min >= max  [%g %g]\n", a, b);

  else do {
    if(++iter == 25) {
      printf("TruncLogNormalRandom: problem with range: ");
      printf("min %g, max %g, ave %g, sd %g\n", a, b, dGeoMean, dGeoSD);
    }
    X = LogNormalRandom(dGeoMean, dGeoSD);
  }
  while (X < a || X > b);

  return X;

} /* TruncLogNormalRandom_old */


/* ----------------------------------------------------------------------------
   TruncNormalRandom

   returns a truncated Normal variate in the range [a, b].
   Uses different rejection sampling algorithms, depending on where the
   upper and lower limits lie. See Robert, Stat. Comp (1995) 5:121-5.
*/
double TruncNormalRandom (double dMean, double dSD, double a, double b)
{
  /* the algorithm works on mean 0, sd 1 scale */
  double lower = (a - dMean) / dSD;
  double upper = (b - dMean) / dSD;
  double dTmp, y;

  if (a >= b) {
    printf ("Error: TruncNormalRandom: min >= max  [%g %g]\n", a, b);
    exit (0);
  }

  if (((lower < 0) && (upper > 0) && (upper - lower > SQRT_2PI))) {
    /* wide range: use the standard normal proposal */
    do {
      y = NormalRandom(0, 1);
    } while ((y < lower) || (y > upper));
  }
  else {
    dTmp = lower + sqrt(lower * lower + 4);
    if ((lower >= 0) &&
        (upper > lower + TWO_SQRTEXP1 / dTmp *
                         exp(lower * (2 - sqrt(lower*lower + 4)) * 0.25))) {
      /* lower >> mean: rejection sampling with exponential proposal */
      dTmp = dTmp * 0.5;
      do {
        y = ExpRandom(dTmp) + lower;
      } while ((Randoms() > exp(-(y - dTmp) * (y - dTmp) * 0.5)) ||
               (y > upper));
    }
    else {
      dTmp = -upper + sqrt(upper * upper + 4);
      if ((upper <= 0) &&
          (lower < upper - TWO_SQRTEXP1 / dTmp *
                           exp(upper * (2 + sqrt(upper*upper + 4)) * 0.25))) {
        /* upper << mean: rejection sampling with exponential proposal */
        dTmp = dTmp * 0.5;
        do {
          y = ExpRandom(dTmp) - upper;
        } while ((Randoms() > exp(-(y - dTmp) * (y - dTmp) * 0.5)) ||
                 (y > -lower));
        y = -y;
      }
      else {
        /* range narrow and central: rejection with uniform proposal */
        do {
          y = UniformRandom(lower, upper);
          if (lower > 0) {
            dTmp = exp((lower * lower - y * y) * 0.5);
          }
          else {
            if (upper < 0) {
              dTmp = exp((upper * upper - y * y) * 0.5);
            } else {
              dTmp = exp(-y * y * 0.5);
            }
          }
        } while (Randoms() > dTmp);
      }
    }
  }

  return(y * dSD + dMean);

} /* TruncNormalRandom */


/* ----------------------------------------------------------------------------
   TruncNormalRandom_old

   returns a truncated Normal variate in the range [a, b].
   Deprecated.
*/
double TruncNormalRandom_old (double dMean, double dSD, double a, double b)
{
  double X = 0.0;
  double unif_density, normConstant, imp_ratio;

  if (a >= b) {
    printf ("Error: TruncNormalRandom: min >= max  [%g %g]\n", a, b);
    exit (0);
  }

  else { /* proceed */
    if ((b - a) / dSD > 1.5) { /* wide truncation range relative to SD */
      do { /* rejection sampling from normal proposal */
        X = NormalRandom(dMean, dSD);
      } while (X < a || X > b);
    }
    else { /* narrow truncation range relative to SD */
      /* normalization constant of the truncated normal */
      normConstant = CDFNormal((b - dMean) / dSD) -
                     CDFNormal((a - dMean) / dSD);
      /* density of any X under a normalized uniform */
      unif_density = (dMean < a ?
                      DFNormal(a, dMean, dSD) / normConstant :
                      (dMean < b ?
                       DFNormal(dMean, dMean, dSD) / normConstant :
                       DFNormal(b, dMean, dSD) / normConstant));
      do { /* rejection sampling from uniform proposal */
        X = UniformRandom (a, b);
        /* density of X under the normalized truncated normal */
        imp_ratio = DFNormal(X, dMean, dSD) /
                    (normConstant * unif_density);
      } while ((imp_ratio < 1) && (Randoms() > imp_ratio));
    }
  }

  return X;

} /* TruncNormalRandom_old */


/* ----------------------------------------------------------------------------
   UniformRandom

   returns a variate that is uniformly distributed on the interval [a,b].
*/
double UniformRandom (double a, double b)
{

  if (b < a) {
    printf ("Error: bad range a for uniform variate - Exiting\n\n");
    exit (0);
  }

  return (Randoms() * (b - a) + a);

} /* UniformRandom */


/* ----------------------------------------------------------------------------
   Wishart

   samples a matrix according to the Wishart distribution by the method
   of Odell and Feiveson (1966).

   Paramters are:
   n (degrees of freedom); p (dimension of Wishart matrix);
   t (pointer to a Cholesky decomposition of a covariance matrix);
   w (pointer to the sampled Wishart matrix, in
   triangular form; work (pointer to a work space, of length p*p).

   Triangular matrices are stored in order
   0 1 3
     2 4
       5 etc.
*/
void WishartRandom (long n, long p, double *t, double *w, double *work)
{
  double eta, sum;
  long i, j, k, m, k1, k2, k3;

  printf ("WishartRandom not tested - Exiting...");
  exit(0);

  /* generate random quantities for Bartlett's decomposition */
  for (j = 0, k = 0; j < p; j++) {
    for (i = 0; i < j; i++)
      w[k++] = NormalRandom(0, 1);

    /* Chi-square with n-i degrees of freedom */
    w[k++] = GGammaRandom((n - i) / 2.0, 0.5);
  }

  /* generate a standard Wishart */
  for (j = p - 1, m = k - 1, k2 = (p * (p - 1)) / 2; j >= 0; k2 = k2 - (j--)) {
    eta = w[m];
    for (i = j, k1 = (i * (i + 1)) / 2; i >= 0; k1 = k1 - (i--), m--) {
      for (k = 0, sum = 0.0; k < i; k++)
        sum = sum + w[k1+k] * w[k2+k];

      if (i == j)
        w[m] = sum + eta;
      else
        w[m] = sum + sqrt(eta) * w[m];
    }
  }

  /* form product L * W * L' */
  for (i = 0, k1 = 0, m = 0; i < p; k1 = k1 + (++i)) {
    for (j = 0, k2 = 0; j < p; k2 = k2 + (++j), m++) {
      for (k = 0, sum = 0.0; k < j; k++)
        sum = sum + t[k1+k] * w[k2+k];

      for (k = j, k3 = j; k <= i; k3 = k3 + (++k))
        sum = sum + t[k1+k] * w[k2+k3];

      work[m] = sum;
    }
  }

  for (i = 0, m = 0, k1 = 0; i < p; i++, k1 = k1 + p) {
    for (j = 0, k2 = 0; j <= i; k2 = k2 + (++j), m++) {
      for (k = 0, sum = 0.0; k <= j; k++)
        sum = sum + work[k1+k] * t[k2+k];

      w[m] = sum;
    }
  }

} /* WishartRandom */


/* ----------------------------------------------------------------------------
   Utility functions
*/

/* ----------------------------------------------------------------------------
   Boolean "AND" logical function.

   Return TRUE if both its two arguments are TRUE. This is just for
   compatibility with SBML.
*/
BOOL and (BOOL A, BOOL B)
{
  return (A && B);
} /* piecewise */


/* ----------------------------------------------------------------------------
   CalcCumulative

   Approximates to an iOrder the cumulative distribution rg_Cdf
   given a sampling grid (rg_x) of dimension cDim, and the
   sampled pdf (rg_pdf) points.

   Supports piecewise-constant (order 0) and piecewise-linear
   (order 1) pdfs.
*/
void CalcCumulative (long cDim, double *rg_x, double *rg_pdf,
                     double *rg_Cdf, int  iOrder)
{
  long i;                /* Index for the samples */

  if (iOrder > 1) {
    printf ("CalcCumulative: Order %d not supported"
                     "-> using piecewise-linear\n", iOrder);
    iOrder = 1;
  }

  rg_Cdf[0] = 0.0;  /* Cumulative starts at 0.0 */
  switch (iOrder) {

  /* Piecewise Constant: sum of rectangles */
  case 0:
    for (i = 1; i < cDim; i++)
      rg_Cdf[i] = rg_Cdf[i-1] + rg_pdf[i]*(rg_x[i] - rg_x[i-1]);
    break;

  /* Piecewise Linear: sum of trapezoids */
  case 1:
    for (i = 1; i < cDim; i++)
      rg_Cdf[i] = rg_Cdf[i-1] + ((rg_x[i] - rg_x[i - 1]) *
                  (rg_pdf[i] + rg_pdf[i - 1]) / 2);
    break;

  default:
    assert (0); /* This is an error condition */
    break;

  } /* switch */

} /* CalcCumulative */


/* ----------------------------------------------------------------------------
   CDFNormal

   the probability for [-inf;Z] under the normal distribution
*/
double CDFNormal (double z)
{
  register double tmp;

  tmp = z/SQRT_2;

  /* avoid roundoff errors generated by subtracting a small number from
     a much bigger one */
  if (tmp >= 0)
    return ( 0.5 * (2 - erfc(tmp)) );
  else
    return ( 0.5 * erfc(-tmp));
}


/* ----------------------------------------------------------------------------
   erfc

   the complementary error function of z (= 1 - erf(z))

   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al.
*/
double erfc (double x)
{
  double dAbsX, t, dVal;

  dAbsX = fabs(x);
  if (dAbsX > 20) { /* FB 16/2/99) */
    return ( x >= 0 ? 0 : 2 );
  }
  else {
    t = 1 / (1 + 0.5 * dAbsX);
    dVal = t * exp(-dAbsX*dAbsX - 1.26551223 + t*(1.00002368 + t*(0.37409196 +
           t*(0.09678418 + t*(-0.18628806 + t*(0.27886807 + t*(-1.13520398 +
           t*(1.48851587 + t*(-0.82215223 + t*(0.17087277))))))))));
    return ( x >= 0 ? dVal : 2 - dVal );
  }
} /* erfc */


/* ----------------------------------------------------------------------------
   DFNormal
   Normal density function
*/
double DFNormal (double x, double mu, double sd)
{
  if (sd <= 0.0) {
    printf ("Error: negative or null SD in DFNormal\n");
    exit (0);
  }

  double tmp = (mu - x) / sd;
  return ( (INV_SQRT_2PI / sd) * exp(-0.5 * tmp * tmp) );
}


/* ----------------------------------------------------------------------------
   InterpolateX

   Do a linear interpolation to return x
*/
double InterpolateX (double rgX[], double rgY[], long lLower, double dY)
{
  return rgX[lLower] + (dY - rgY[lLower]) *
         (rgX[lLower + 1] - rgX[lLower]) /
         (rgY[lLower + 1] - rgY[lLower]);

} /* InterpolateX */


/* ----------------------------------------------------------------------------
   lnDFBeta
   the log of the beta density function
   FB 08/07/97
*/
double lnDFBeta (double x, double alpha, double beta, double min, double max)
{
  if (max <= min) {
    printf ("Error: bad range for beta variate in lnDFBeta\n");
    exit (0);
  }
  if (alpha <= 0) {
    printf ("Error: bad alpha for beta variate in LnDensity\n");
    exit (0);
  }
  if (beta <= 0) {
    printf ("Error: bad beta for beta variate in LnDensity\n");
    exit (0);
  }

  x = (x - min) / (max - min);
  return (alpha - 1) * log (x) + (beta - 1) * log (1 - x) +
         lnGamma (alpha + beta) - lnGamma (alpha) - lnGamma(beta) -
         log (max - min);
}


/* ----------------------------------------------------------------------------
   lnDFNormal
   log of the normal density function
*/
double lnDFNormal (double x, double mu, double sd)
{
  if (sd <= 0.0) {
    printf ("Error: negative or null SD in lnDFNormal\n");
    exit (0);
  }

  double tmp = (mu - x) / sd;
  return ( -LOG_SQRT_2PI - log(sd) - 0.5 * tmp * tmp );
}


/* ----------------------------------------------------------------------------
   lnGamma

   A function to return the natural log of the Gamma function of x.
   Adapted from the algorithm described in the book Numerical Recipes by
   Press et al.
   It can be used to compute factorials since ln(n!) = lnGamma(n + 1)
*/
double lnGamma (double x)
{
  double dSeries, dTemp;

  if (x <= 0.0) {
    printf ("Error: negative or null parameter for lnGamma function\n");
    exit (0);
  }

  dSeries = 1.000000000190015 +
            76.18009172947146   /  x      -
            86.50532032141677   / (x + 1) +
            24.01409824083091   / (x + 2) -
            1.231739572450155   / (x + 3) +
            1.20865097386617E-3 / (x + 4) -
            5.39523938495E-6    / (x + 5);

  dTemp = x + 4.5;
  dTemp = -dTemp + (x - 0.5) * log (dTemp) + log (2.50662827465 * dSeries);
  return dTemp;

} /* lnGamma */


#ifdef ndef
/* ----------------------------------------------------------------------------
   Piecewise math function.

   Takes an unknwn number of pairs of (double, boolean), and eventually a
   terminal double. Returns the first double value for which the associated
   condition is true. If none is true it returns FALSE or the terminal double
   if one is given.
   Example: {double am = 1., pm = 2., noon = 3., time = 55.888;
             return piecewise (am, (time < 12), pm, (time > 12), noon);}
*/
void piecewise (double X1 ...)
{
  va_list argptr;

  va_start(argprt, X1);

  if ((arg1 = va_arg (argptr, char*)) != NULL) {
    test = arg1;
    while ((arg[++nargs] = va_arg(ap, char*)) != NULL) {};
  }
  va_end (ap);

  /* get a target - clause pair
    szMsg1 = va_arg(ap, PSTR);
    szMsg2 = va_arg(ap, PSTR); */
  va_end(ap);

} /* piecewise */
#endif


/* End of random module */
