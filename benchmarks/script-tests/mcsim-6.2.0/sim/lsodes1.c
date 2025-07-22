/* lsodes1.c

   Copyright (c) 1993-2017 Free Software Foundation, Inc.

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

   lsodes.c was translated from lsodes.f by the utility f2c.
   To make lsodes.c a stand alone C routine, the following modifications 
   were made:
        1. the options -lF77 -lI77 were removed from the link command line
        2. a function d_sign was written and added at the beginning of
           the function body
        3. lsodes was cut in two pieces

  This is the first file for the two parts

  -----------------------------------------------------------------------------
  LSODES - summary of usage.

  This is the march 30, 1987 version of lsodes -
  The Livermore solver for ordinary differential equations
  with general sparse jacobian matrices.
  This version is in double precision.
  
  Communication between the user and the lsodes package, for normal
  situations, is summarized here.  this summary describes only a subset
  of the full set of options available.  see the full description for
  details, including optional communication, nonstandard options,
  and instructions for special situations.  see also the example
  problem (with program and output) following this summary.
  
  a. first provide a subroutine of the form..
                subroutine CalcDeriv (neq, t, y, ydot)
                dimension y(neq), ydot(neq)
  which supplies the vector function f by loading ydot(i) with f(i).
  
  b. next determine (or guess) whether or not the problem is stiff.
  stiffness occurs when the jacobian matrix df/dy has an eigenvalue
  whose real part is negative and large in magnitude, compared to the
  reciprocal of the t span of interest.  if the problem is nonstiff,
  use a method flag mf = 10.  if it is stiff, there are two standard
  for the method flag, mf = 121 and mf = 222.  in both cases, lsodes
  requires the jacobian matrix in some form, and it treats this matrix
  in general sparse form, with sparsity structure determined internally.
  (for options where the user supplies the sparsity structure, see
  the full description of mf below.)
  
  c. if the problem is stiff, you are encouraged to supply the jacobian
  directly (mf = 121), but if this is not feasible, lsodes will
  compute it internally by difference quotients (mf = 222).
  if you are supplying the jacobian, provide a subroutine of the form..
                subroutine jac (neq, t, y, j, ian, jan, pdj)
                (here CalcJacob (t, y, j, pdj))
                dimension y(1), ian(1), jan(1), pdj(1)
  here neq, t, y, and j are input arguments, and the jac routine is to
  load the array pdj (of length neq) with the j-th column of df/dy.
  i.e., load pdj(i) with df(i)/dy(j) for all relevant values of i.
  the arguments ian and jan should be ignored for normal situations.
  lsodes will call the jac routine with j = 1,2,...,neq.
  only nonzero elements need be loaded.  usually, a crude approximation
  to df/dy, possibly with fewer nonzero elements, will suffice.
  
  d. write a main program which calls subroutine lsodes once for
  each point at which answers are desired.  this should also provide
  for possible use of logical unit 6 for output of error messages
  by lsodes.  on the first call to lsodes, supply arguments as follows..
  f      = name of subroutine for right-hand side vector f.
           this name must be declared external in calling program.
  neq    = number of first order ode-s.
  y      = array of initial values, of length neq.
  t      = the initial value of the independent variable.
  tout   = first point where output is desired (.ne. t).
  itol   = 1 or 2 according as atol (below) is a scalar or array.
  rtol   = relative tolerance parameter (scalar).
  atol   = absolute tolerance parameter (scalar or array).
           the estimated local error in y(i) will be controlled so as
           to be roughly less (in magnitude) than
              ewt(i) = rtol*abs(y(i)) + atol     if itol = 1, or
              ewt(i) = rtol*abs(y(i)) + atol(i)  if itol = 2.
           thus the local error test passes if, in each component,
           either the absolute error is less than atol (or atol(i)),
           or the relative error is less than rtol.
           use rtol = 0.0 for pure absolute error control, and
           use atol = 0.0 (or atol(i) = 0.0) for pure relative error
           control.  caution.. actual (global) errors may exceed these
           local tolerances, so choose them conservatively.
  itask  = 1 for normal computation of output values of y at t = tout.
  istate = integer flag (input and output).  set istate = 1.
  iopt   = 0 to indicate no optional inputs used.
  rwork  = real work array of length at least..
              20 + 16*neq            for mf = 10,
              20 + (2 + 1./lenrat)*nnz + (11 + 9./lenrat)*neq
                                     for mf = 121 or 222,
           where..
           nnz    = the number of nonzero elements in the sparse
                    jacobian (if this is unknown, use an estimate), and
           lenrat = the real to integer wordlength ratio (usually 1 in
                    single precision and 2 in double precision).
           in any case, the required size of rwork cannot generally
           be predicted in advance if mf = 121 or 222, and the value
           above is a rough estimate of a crude lower bound.  some
           experimentation with this size may be necessary.
           (when known, the correct required length is an optional
           output, available in iwork(17).)
  lrw    = declared length of rwork (in user-s dimension).
  iwork  = integer work array of length at least 30.
  liw    = declared length of iwork (in user-s dimension).
  jac    = name of subroutine for Jacobian matrix (mf = 121).
           if used, this name must be declared external in calling
           program.  if not used, pass a dummy name.
  mf     = method flag.  standard values are..
           10  for nonstiff (adams) method, no Jacobian used.
           121 for stiff (bdf) method, user-supplied sparse Jacobian.
           222 for stiff method, internally generated sparse Jacobian.
  note that the main program must declare arrays y, rwork, iwork,
  and possibly atol.
  
  e. the output from the first call (or any call) is..
       y = array of computed values of y(t) vector.
       t = corresponding value of independent variable (normally tout).
  istate = 2  if lsodes was successful, negative otherwise.
           -1 means excess work done on this call (perhaps wrong mf).
           -2 means excess accuracy requested (tolerances too small).
           -3 means illegal input detected (see printed message).
           -4 means repeated error test failures (check all inputs).
           -5 means repeated convergence failures (perhaps bad Jacobian
              supplied or wrong choice of mf or tolerances).
           -6 means error weight became zero during problem. (solution
              component i vanished, and atol or atol(i) = 0.)
           -7 means a fatal error return flag came from the sparse
              solver cdrv by way of prjs or slss.  should never happen.
           a return with istate = -1, -4, or -5 may result from using
           an inappropriate sparsity structure, one that is quite
           different from the initial structure.  consider calling
           lsodes again with istate = 3 to force the structure to be
           reevaluated.  see the full description of istate below.
  
  f. to continue the integration after a successful return, simply
  reset tout and call lsodes again.  no other parameters need be reset.
  
  -----------------------------------------------------------------------------
  full description of user interface to lsodes.
  
  the user interface to lsodes consists of the following parts.
  
  i.   the call sequence to subroutine lsodes, which is a driver
       routine for the solver.  this includes descriptions of both
       the call sequence arguments and of user-supplied routines.
       following these descriptions is a description of
       optional inputs available through the call sequence, and then
       a description of optional outputs (in the work arrays).
  
  ii.  descriptions of other routines in the lsodes package that may be
       (optionally) called by the user.  these provide the ability to
       alter error message handling, save and restore the internal
       common, and obtain specified derivatives of the solution y(t).
  
  iii. descriptions of common blocks to be declared in overlay
       or similar environments, or to be saved when doing an interrupt
       of the problem and continued solution later.
  
  iv.  description of two routines in the lsodes package, either of
       which the user may replace with his own version, if desired.
       these relate to the measurement of errors.
  
  -----------------------------------------------------------------------------
  part i.  call sequence.
  
  the call sequence parameters used for input only are
      f, neq, tout, itol, rtol, atol, itask, iopt, lrw, liw, jac, mf,
  and those used for both input and output are
      y, t, istate.
  the work arrays rwork and iwork are also used for conditional and
  optional inputs and optional outputs.  (the term output here refers
  to the return from subroutine lsodes to the user-s calling program.)
  
  the legality of input parameters will be thoroughly checked on the
  initial call for the problem, but not checked thereafter unless a
  change in input parameters is flagged by istate = 3 on input.
  
  the descriptions of the call arguments are as follows.
  
  f      = the name of the user-supplied subroutine defining the
           ode system.  the system must be put in the first-order
           form dy/dt = f(t,y), where f is a vector-valued function
           of the scalar t and the vector y.  subroutine f is to
           compute the function f.  it is to have the form
                subroutine f (neq, t, y, ydot)
                dimension y(1), ydot(1)
           where neq, t, and y are input, and the array ydot = f(t,y)
           is output.  y and ydot are arrays of length neq.
           (in the dimension statement above, 1 is a dummy
           dimension.. it can be replaced by any value.)
           subroutine f should not alter y(1),...,y(neq).
           f must be declared external in the calling program.
  
           subroutine f may access user-defined quantities in
           neq(2),... and/or in y(neq(1)+1),... if neq is an array
           (dimensioned in f) and/or y has length exceeding neq(1).
           see the descriptions of neq and y below.
  
           if quantities computed in the f routine are needed
           externally to lsodes, an extra call to f should be made
           for this purpose, for consistent and accurate results.
           if only the derivative dy/dt is needed, use intdy instead.
  
  neq    = the size of the ode system (number of first order
           ordinary differential equations).  used only for input.
           neq may be decreased, but not increased, during the problem.
           if neq is decreased (with istate = 3 on input), the
           remaining components of y should be left undisturbed, if
           these are to be accessed in f and/or jac.
  
           normally, neq is a scalar, and it is generally referred to
           as a scalar in this user interface description.  however,
           neq may be an array, with neq(1) set to the system size.
           (the lsodes package accesses only neq(1).)  in either case,
           this parameter is passed as the neq argument in all calls
           to f and jac.  hence, if it is an array, locations
           neq(2),... may be used to store other integer data and pass
           it to f and/or jac.  subroutines f and/or jac must include
           neq in a dimension statement in that case.
  
  y      = a real array for the vector of dependent variables, of
           length neq or more.  used for both input and output on the
           first call (istate = 1), and only for output on other calls.
           on the first call, y must contain the vector of initial
           values.  on output, y contains the computed solution vector,
           evaluated at t.  if desired, the y array may be used
           for other purposes between calls to the solver.
  
           this array is passed as the y argument in all calls to
           f and jac.  hence its length may exceed neq, and locations
           y(neq+1),... may be used to store other real data and
           pass it to f and/or jac.  (the lsodes package accesses only
           y(1),...,y(neq).)
  
  t      = the independent variable.  on input, t is used only on the
           first call, as the initial point of the integration.
           on output, after each call, t is the value at which a
           computed solution y is evaluated (usually the same as tout).
           on an error return, t is the farthest point reached.
  
  tout   = the next value of t at which a computed solution is desired.
           used only for input.
  
           when starting the problem (istate = 1), tout may be equal
           to t for one call, then should .ne. t for the next call.
           for the initial t, an input value of tout .ne. t is used
           in order to determine the direction of the integration
           (i.e. the algebraic sign of the step sizes) and the rough
           scale of the problem.  integration in either direction
           (forward or backward in t) is permitted.
  
           if itask = 2 or 5 (one-step modes), tout is ignored after
           the first call (i.e. the first call with tout .ne. t).
           otherwise, tout is required on every call.
  
           if itask = 1, 3, or 4, the values of tout need not be
           monotone, but a value of tout which backs up is limited
           to the current internal t interval, whose endpoints are
           tcur - hu and tcur (see optional outputs, below, for
           tcur and hu).
  
  itol   = an indicator for the type of error control.  see
           description below under atol.  used only for input.
  
  rtol   = a relative error tolerance parameter, either a scalar or
           an array of length neq.  see description below under atol.
           input only.
  
  atol   = an absolute error tolerance parameter, either a scalar or
           an array of length neq.  input only.
  
              the input parameters itol, rtol, and atol determine
           the error control performed by the solver.  the solver will
           control the vector e = (e(i)) of estimated local errors
           in y, according to an inequality of the form
                       rms-norm of ( e(i)/ewt(i) )   .le.   1,
           where       ewt(i) = rtol(i)*abs(y(i)) + atol(i),
           and the rms-norm (root-mean-square norm) here is
           rms-norm(v) = sqrt(sum v(i)**2 / neq).  here ewt = (ewt(i))
           is a vector of weights which must always be positive, and
           the values of rtol and atol should all be non-negative.
           the following table gives the types (scalar/array) of
           rtol and atol, and the corresponding form of ewt(i).
  
              itol    rtol       atol          ewt(i)
               1     scalar     scalar     rtol*abs(y(i)) + atol
               2     scalar     array      rtol*abs(y(i)) + atol(i)
               3     array      scalar     rtol(i)*abs(y(i)) + atol
               4     array      array      rtol(i)*abs(y(i)) + atol(i)
  
           when either of these parameters is a scalar, it need not
           be dimensioned in the user-s calling program.
  
           if none of the above choices (with itol, rtol, and atol
           fixed throughout the problem) is suitable, more general
           error controls can be obtained by substituting
           user-supplied routines for the setting of ewt and/or for
           the norm calculation.  see part iv below.
  
           if global errors are to be estimated by making a repeated
           run on the same problem with smaller tolerances, then all
           components of rtol and atol (i.e. of ewt) should be scaled
           down uniformly.
  
  itask  = an index specifying the task to be performed.
           input only.  itask has the following values and meanings.
           1  means normal computation of output values of y(t) at
              t = tout (by overshooting and interpolating).
           2  means take one step only and return.
           3  means stop at the first internal mesh point at or
              beyond t = tout and return.
           4  means normal computation of output values of y(t) at
              t = tout but without overshooting t = tcrit.
              tcrit must be input as rwork(1).  tcrit may be equal to
              or beyond tout, but not behind it in the direction of
              integration.  this option is useful if the problem
              has a singularity at or beyond t = tcrit.
           5  means take one step, without passing tcrit, and return.
              tcrit must be input as rwork(1).
  
           note..  if itask = 4 or 5 and the solver reaches tcrit
           (within roundoff), it will return t = tcrit (exactly) to
           indicate this (unless itask = 4 and tout comes before tcrit,
           in which case answers at t = tout are returned first).
  
  istate = an index used for input and output to specify the
           the state of the calculation.
  
           on input, the values of istate are as follows.
           1  means this is the first call for the problem
              (initializations will be done).  see note below.
           2  means this is not the first call, and the calculation
              is to continue normally, with no change in any input
              parameters except possibly tout and itask.
              (if itol, rtol, and/or atol are changed between calls
              with istate = 2, the new values will be used but not
              tested for legality.)
           3  means this is not the first call, and the
              calculation is to continue normally, but with
              a change in input parameters other than
              tout and itask.  changes are allowed in
              neq, itol, rtol, atol, iopt, lrw, liw, mf,
              the conditional inputs ia and ja,
              and any of the optional inputs except h0.
              in particular, if miter = 1 or 2, a call with istate = 3
              will cause the sparsity structure of the problem to be
              recomputed (or reread from ia and ja if moss = 0).
           note..  a preliminary call with tout = t is not counted
           as a first call here, as no initialization or checking of
           input is done.  (such a call is sometimes useful for the
           purpose of outputting the initial conditions.)
           thus the first call for which tout .ne. t requires
           istate = 1 on input.
  
           on output, istate has the following values and meanings.
            1  means nothing was done, as tout was equal to t with
               istate = 1 on input.  (however, an internal counter was
               set to detect and prevent repeated calls of this type.)
            2  means the integration was performed successfully.
           -1  means an excessive amount of work (more than mxstep
               steps) was done on this call, before completing the
               requested task, but the integration was otherwise
               successful as far as t.  (mxstep is an optional input
               and is normally 500.)  to continue, the user may
               simply reset istate to a value .gt. 1 and call again
               (the excess work step counter will be reset to 0).
               in addition, the user may increase mxstep to avoid
               this error return (see below on optional inputs).
           -2  means too much accuracy was requested for the precision
               of the machine being used.  this was detected before
               completing the requested task, but the integration
               was successful as far as t.  to continue, the tolerance
               parameters must be reset, and istate must be set
               to 3.  the optional output tolsf may be used for this
               purpose.  (note.. if this condition is detected before
               taking any steps, then an illegal input return
               (istate = -3) occurs instead.)
           -3  means illegal input was detected, before taking any
               integration steps.  see written message for details.
               note..  if the solver detects an infinite loop of calls
               to the solver with illegal input, it will cause
               the run to stop.
           -4  means there were repeated error test failures on
               one attempted step, before completing the requested
               task, but the integration was successful as far as t.
               the problem may have a singularity, or the input
               may be inappropriate.
           -5  means there were repeated convergence test failures on
               one attempted step, before completing the requested
               task, but the integration was successful as far as t.
               this may be caused by an inaccurate Jacobian matrix,
               if one is being used.
           -6  means ewt(i) became zero for some i during the
               integration.  pure relative error control (atol(i)=0.0)
               was requested on a variable which has now vanished.
               the integration was successful as far as t.
           -7  means a fatal error return flag came from the sparse
               solver cdrv by way of prjs or slss (numerical
               factorization or backsolve).  this should never happen.
               the integration was successful as far as t.
  
           note.. an error return with istate = -1, -4, or -5 and with
           miter = 1 or 2 may mean that the sparsity structure of the
           problem has changed significantly since it was last
           determined (or input).  in that case, one can attempt to
           complete the integration by setting istate = 3 on the next
           call, so that a new structure determination is done.
  
           note..  since the normal output value of istate is 2,
           it does not need to be reset for normal continuation.
           also, since a negative input value of istate will be
           regarded as illegal, a negative output value requires the
           user to change it, and possibly other inputs, before
           calling the solver again.
  
  iopt   = an integer flag to specify whether or not any optional
           inputs are being used on this call.  input only.
           the optional inputs are listed separately below.
           iopt = 0 means no optional inputs are being used.
                    default values will be used in all cases.
           iopt = 1 means one or more optional inputs are being used.
  
  rwork  = a work array used for a mixture of real (double precision)
           and integer work space.
           the length of rwork (in real words) must be at least
              20 + nyh*(maxord + 1) + 3*neq + lwm    where
           nyh    = the initial value of neq,
           maxord = 12 (if meth = 1) or 5 (if meth = 2) (unless a
                    smaller value is given as an optional input),
           lwm = 0                                    if miter = 0,
           lwm = 2*nnz + 2*neq + (nnz+9*neq)/lenrat   if miter = 1,
           lwm = 2*nnz + 2*neq + (nnz+10*neq)/lenrat  if miter = 2,
           lwm = neq + 2                              if miter = 3.
           in the above formulas,
           nnz    = number of nonzero elements in the Jacobian matrix.
           lenrat = the real to integer wordlength ratio (usually 1 in
                    single precision and 2 in double precision).
           (see the mf description for meth and miter.)
           thus if maxord has its default value and neq is constant,
           the minimum length of rwork is..
              20 + 16*neq        for mf = 10,
              20 + 16*neq + lwm  for mf = 11, 111, 211, 12, 112, 212,
              22 + 17*neq        for mf = 13,
              20 +  9*neq        for mf = 20,
              20 +  9*neq + lwm  for mf = 21, 121, 221, 22, 122, 222,
              22 + 10*neq        for mf = 23.
           if miter = 1 or 2, the above formula for lwm is only a
           crude lower bound.  the required length of rwork cannot
           be readily predicted in general, as it depends on the
           sparsity structure of the problem.  some experimentation
           may be necessary.
  
           the first 20 words of rwork are reserved for conditional
           and optional inputs and optional outputs.
  
           the following word in rwork is a conditional input..
             rwork(1) = tcrit = critical value of t which the solver
                        is not to overshoot.  required if itask is
                        4 or 5, and ignored otherwise.  (see itask.)
  
  lrw    = the length of the array rwork, as declared by the user.
           (this will be checked by the solver.)
  
  iwork  = an integer work array.  the length of iwork must be at least
              31 + neq + nnz   if moss = 0 and miter = 1 or 2, or
              30               otherwise.
           (nnz is the number of nonzero elements in df/dy.)
  
           in lsodes, iwork is used only for conditional and
           optional inputs and optional outputs.
  
           the following two blocks of words in iwork are conditional
           inputs, required if moss = 0 and miter = 1 or 2, but not
           otherwise (see the description of mf for moss).
             iwork(30+j) = ia(j)     (j=1,...,neq+1)
             iwork(31+neq+k) = ja(k) (k=1,...,nnz)
           the two arrays ia and ja describe the sparsity structure
           to be assumed for the Jacobian matrix.  ja contains the row
           indices where nonzero elements occur, reading in columnwise
           order, and ia contains the starting locations in ja of the
           descriptions of columns 1,...,neq, in that order, with
           ia(1) = 1.  thus, for each column index j = 1,...,neq, the
           values of the row index i in column j where a nonzero
           element may occur are given by
             i = ja(k),  where   ia(j) .le. k .lt. ia(j+1).
           if nnz is the total number of nonzero locations assumed,
           then the length of the ja array is nnz, and ia(neq+1) must
           be nnz + 1.  duplicate entries are not allowed.
  
  liw    = the length of the array iwork, as declared by the user.
           (this will be checked by the solver.)
  
  Note..  the work arrays must not be altered between calls to lsodes
  for the same problem, except possibly for the conditional and
  optional inputs, and except for the last 3*neq words of rwork.
  the latter space is used for internal scratch space, and so is
  available for use by the user outside lsodes between calls, if
  desired (but not for use by f or jac).
  
  jac    = name of user-supplied routine (miter = 1 or moss = 1) to
           compute the Jacobian matrix, df/dy, as a function of
           the scalar t and the vector y.  It is to have the form
                subroutine jac (neq, t, y, j, ian, jan, pdj)
                dimension y(1), ian(1), jan(1), pdj(1)
           where neq, t, y, j, ian, and jan are input, and the array
           pdj, of length neq, is to be loaded with column j
           of the Jacobian on output.  Thus df(i)/dy(j) is to be
           loaded into pdj(i) for all relevant values of i.
           here t and y have the same meaning as in subroutine f,
           and j is a column index (1 to neq).  ian and jan are
           undefined in calls to jac for structure determination
           (moss = 1).  otherwise, ian and jan are structure
           descriptors, as defined under optional outputs below, and
           so can be used to determine the relevant row indices i, if
           desired.  (in the dimension statement above, 1 is a
           dummy dimension.. it can be replaced by any value.)
                jac need not provide df/dy exactly.  a crude
           approximation (possibly with greater sparsity) will do.
                in any case, pdj is preset to zero by the solver,
           so that only the nonzero elements need be loaded by jac.
           calls to jac are made with j = 1,...,neq, in that order, and
           each such set of calls is preceded by a call to f with the
           same arguments neq, t, and y.  thus to gain some efficiency,
           intermediate quantities shared by both calculations may be
           saved in a user common block by f and not recomputed by jac,
           if desired.  jac must not alter its input arguments.
           jac must be declared external in the calling program.
                subroutine jac may access user-defined quantities in
           neq(2),... and y(neq(1)+1),... if neq is an array
           (dimensioned in jac) and y has length exceeding neq(1).
           see the descriptions of neq and y above.
  
  mf     = the method flag.  used only for input.
           mf has three decimal digits-- moss, meth, miter--
              mf = 100*moss + 10*meth + miter.
           moss indicates the method to be used to obtain the sparsity
           structure of the jacobian matrix if miter = 1 or 2..
             moss = 0 means the user has supplied ia and ja
                      (see descriptions under iwork above).
             moss = 1 means the user has supplied jac (see below)
                      and the structure will be obtained from neq
                      initial calls to jac.
             moss = 2 means the structure will be obtained from neq+1
                      initial calls to f.
           meth indicates the basic linear multistep method..
             meth = 1 means the implicit adams method.
             meth = 2 means the method based on backward
                      differentiation formulas (bdf-s).
           miter indicates the corrector iteration method..
             miter = 0 means functional iteration (no jacobian matrix
                       is involved).
             miter = 1 means chord iteration with a user-supplied
                       sparse jacobian, given by subroutine jac.
             miter = 2 means chord iteration with an internally
                       generated (difference quotient) sparse jacobian
                       (using ngp extra calls to f per df/dy value,
                       where ngp is an optional output described below.)
             miter = 3 means chord iteration with an internally
                       generated diagonal jacobian approximation.
                       (using 1 extra call to f per df/dy evaluation).
           if miter = 1 or moss = 1, the user must supply a subroutine
           jac (the name is arbitrary) as described above under jac.
           otherwise, a dummy argument can be used.
  
           the standard choices for mf are..
             mf = 10  for a nonstiff problem,
             mf = 21 or 22 for a stiff problem with ia/ja supplied
                      (21 if jac is supplied, 22 if not),
             mf = 121 for a stiff problem with jac supplied,
                      but not ia/ja,
             mf = 222 for a stiff problem with neither ia/ja nor
                      jac supplied.
           the sparseness structure can be changed during the
           problem by making a call to lsodes with istate = 3.
  
  -----------------------------------------------------------------------------
  optional inputs.
  
  the following is a list of the optional inputs provided for in the
  call sequence.  (see also part ii.)  for each such input variable,
  this table lists its name as used in this documentation, its
  location in the call sequence, its meaning, and the default value.
  the use of any of these inputs requires iopt = 1, and in that
  case all of these inputs are examined.  a value of zero for any
  of these optional inputs will cause the default value to be used.
  thus to use a subset of the optional inputs, simply preload
  locations 5 to 10 in rwork and iwork to 0.0 and 0 respectively, and
  then set those of interest to nonzero values.
  
  name    location  meaning and default value
  
  h0      rwork(5)  the step size to be attempted on the first step.
                    the default value is determined by the solver.
  
  hmax    rwork(6)  the maximum absolute step size allowed.
                    the default value is infinite.
  
  hmin    rwork(7)  the minimum absolute step size allowed.
                    the default value is 0.  (this lower bound is not
                    enforced on the final step before reaching tcrit
                    when itask = 4 or 5.)
  
  seth    rwork(8)  the element threshhold for sparsity determination
                    when moss = 1 or 2.  if the absolute value of
                    an estimated jacobian element is .le. seth, it
                    will be assumed to be absent in the structure.
                    the default value of seth is 0.
  
  maxord  iwork(5)  the maximum order to be allowed.  the default
                    value is 12 if meth = 1, and 5 if meth = 2.
                    if maxord exceeds the default value, it will
                    be reduced to the default value.
                    if maxord is changed during the problem, it may
                    cause the current order to be reduced.
  
  mxstep  iwork(6)  maximum number of (internally defined) steps
                    allowed during one call to the solver.
                    the default value is 500.
  
  mxhnil  iwork(7)  maximum number of messages printed (per problem)
                    warning that t + h = t on a step (h = step size).
                    this must be positive to result in a non-default
                    value.  the default value is 10.
  
  -----------------------------------------------------------------------------
  optional outputs.
  
  as optional additional output from lsodes, the variables listed
  below are quantities related to the performance of lsodes
  which are available to the user.  these are communicated by way of
  the work arrays, but also have internal mnemonic names as shown.
  except where stated otherwise, all of these outputs are defined
  on any successful return from lsodes, and on any return with
  istate = -1, -2, -4, -5, or -6.  on an illegal input return
  (istate = -3), they will be unchanged from their existing values
  (if any), except possibly for tolsf, lenrw, and leniw.
  on any error return, outputs relevant to the error will be defined,
  as noted below.
  
  name    location      meaning
  
  hu      rwork(11) the step size in t last used (successfully).
  
  hcur    rwork(12) the step size to be attempted on the next step.
  
  tcur    rwork(13) the current value of the independent variable
                    which the solver has actually reached, i.e. the
                    current internal mesh point in t.  on output, tcur
                    will always be at least as far as the argument
                    t, but may be farther (if interpolation was done).
  
  tolsf   rwork(14) a tolerance scale factor, greater than 1.0,
                    computed when a request for too much accuracy was
                    detected (istate = -3 if detected at the start of
                    the problem, istate = -2 otherwise).  if itol is
                    left unaltered but rtol and atol are uniformly
                    scaled up by a factor of tolsf for the next call,
                    then the solver is deemed likely to succeed.
                    (the user may also ignore tolsf and alter the
                    tolerance parameters in any other way appropriate.)
  
  nst     iwork(11) the number of steps taken for the problem so far.
  
  nfe     iwork(12) the number of f evaluations for the problem so far,
                    excluding those for structure determination
                    (moss = 2).
  
  nje     iwork(13) the number of jacobian evaluations for the problem
                    so far, excluding those for structure determination
                    (moss = 1).
  
  nqu     iwork(14) the method order last used (successfully).
  
  nqcur   iwork(15) the order to be attempted on the next step.
  
  imxer   iwork(16) the index of the component of largest magnitude in
                    the weighted local error vector ( e(i)/ewt(i) ),
                    on an error return with istate = -4 or -5.
  
  lenrw   iwork(17) the length of rwork actually required.
                    this is defined on normal returns and on an illegal
                    input return for insufficient storage.
  
  leniw   iwork(18) the length of iwork actually required.
                    this is defined on normal returns and on an illegal
                    input return for insufficient storage.
  
  nnz     iwork(19) the number of nonzero elements in the jacobian
                    matrix, including the diagonal (miter = 1 or 2).
                    (this may differ from that given by ia(neq+1)-1
                    if moss = 0, because of added diagonal entries.)
  
  ngp     iwork(20) the number of groups of column indices, used in
                    difference quotient jacobian aproximations if
                    miter = 2.  this is also the number of extra f
                    evaluations needed for each jacobian evaluation.
  
  nlu     iwork(21) the number of sparse lu decompositions for the
                    problem so far.
  
  lyh     iwork(22) the base address in rwork of the history array yh,
                    described below in this list.
  
  ipian   iwork(23) the base address of the structure descriptor array
                    ian, described below in this list.
  
  ipjan   iwork(24) the base address of the structure descriptor array
                    jan, described below in this list.
  
  nzl     iwork(25) the number of nonzero elements in the strict lower
                    triangle of the lu factorization used in the chord
                    iteration (miter = 1 or 2).
  
  nzu     iwork(26) the number of nonzero elements in the strict upper
                    triangle of the lu factorization used in the chord
                    iteration (miter = 1 or 2).
                    the total number of nonzeros in the factorization
                    is therefore nzl + nzu + neq.
  
  the following four arrays are segments of the rwork array which
  may also be of interest to the user as optional outputs.
  for each array, the table below gives its internal name,
  its base address, and its description.
  for yh and acor, the base addresses are in rwork (a real array).
  the integer arrays ian and jan are to be obtained by declaring an
  integer array iwk and identifying iwk(1) with rwork(21), using either
  an equivalence statement or a subroutine call.  then the base
  addresses ipian (of ian) and ipjan (of jan) in iwk are to be obtained
  as optional outputs iwork(23) and iwork(24), respectively.
  thus ian(1) is iwk(ipian), etc.
  
  name    base address      description
  
  ian    ipian (in iwk)  structure descriptor array of size neq + 1.
  jan    ipjan (in iwk)  structure descriptor array of size nnz.
          (see above)    ian and jan together describe the sparsity
                         structure of the jacobian matrix, as used by
                         lsodes when miter = 1 or 2.
                         jan contains the row indices of the nonzero
                         locations, reading in columnwise order, and
                         ian contains the starting locations in jan of
                         the descriptions of columns 1,...,neq, in
                         that order, with ian(1) = 1.  thus for each
                         j = 1,...,neq, the row indices i of the
                         nonzero locations in column j are
                         i = jan(k),  ian(j) .le. k .lt. ian(j+1).
                         note that ian(neq+1) = nnz + 1.
                         (if moss = 0, ian/jan may differ from the
                         input ia/ja because of a different ordering
                         in each column, and added diagonal entries.)
  
  yh      lyh            the nordsieck history array, of size nyh by
           (optional     (nqcur + 1), where nyh is the initial value
           output)       of neq.  for j = 0,1,...,nqcur, column j+1
                         of yh contains hcur**j/factorial(j) times
                         the j-th derivative of the interpolating
                         polynomial currently representing the solution,
                         evaluated at t = tcur.  the base address lyh
                         is another optional output, listed above.
  
  acor     lenrw-neq+1   array of size neq used for the accumulated
                         corrections on each step, scaled on output
                         to represent the estimated local error in y
                         on the last step.  this is the vector e in
                         the description of the error control.  it is
                         defined only on a successful return from
                         lsodes.
  
  -----------------------------------------------------------------------------
  part ii.  other routines callable.
  
  the following are optional calls which the user may make to
  gain additional capabilities in conjunction with lsodes.
  
    call srcms(rsav,isav,job) saves and restores the contents of
                              the internal common blocks used by
                              lsodes (see part iii below).
                              rsav must be a real array of length 224
                              or more, and isav must be an integer
                              array of length 75 or more.
                              job=1 means save common into rsav/isav.
                              job=2 means restore common from rsav/isav.
                                 srcms is useful if one is
                              interrupting a run and restarting
                              later, or alternating between two or
                              more problems solved with lsodes.
  
    call intdy(,,,,,)         provide derivatives of y, of various
         (see below)          orders, at a specified point t, if
                              desired.  it may be called only after
                              a successful return from lsodes.
  
  the detailed instructions for using intdy are as follows.
  the form of the call is..
  
    lyh = iwork(22)
    call intdy (t, k, rwork(lyh), nyh, dky, iflag)
  
  the input parameters are..
  
  t         = value of independent variable where answers are desired
              (normally the same as the t last returned by lsodes).
              for valid results, t must lie between tcur - hu and tcur.
              (see optional outputs for tcur and hu.)
  k         = integer order of the derivative desired.  k must satisfy
              0 .le. k .le. nqcur, where nqcur is the current order
              (see optional outputs).  the capability corresponding
              to k = 0, i.e. computing y(t), is already provided
              by lsodes directly.  since nqcur .ge. 1, the first
              derivative dy/dt is always available with intdy.
  lyh       = the base address of the history array yh, obtained
              as an optional output as shown above.
  nyh       = column length of yh, equal to the initial value of neq.
  
  the output parameters are..
  
  dky       = a real array of length neq containing the computed value
              of the k-th derivative of y(t).
  iflag     = integer flag, returned as 0 if k and t were legal,
              -1 if k was illegal, and -2 if t was illegal.
              on an error return, a message is also written.
  
  -----------------------------------------------------------------------------
  part iii.  common blocks.
  
  if lsodes is to be used in an overlay situation, the user
  must declare, in the primary overlay, the variables in..
    (1) the call sequence to lsodes,
    (2) the three internal common blocks
          /ls0001/  of length  257  (218 double precision words
                          followed by 39 integer words),
          /lss001/  of length  40    ( 6 double precision words
                          followed by 34 integer words),
          /eh0001/  of length  2 (integer words).
  
  if lsodes is used on a system in which the contents of internal
  common blocks are not preserved between calls, the user should
  declare the above three common blocks in his main program to insure
  that their contents are preserved.
  
  if the solution of a given problem by lsodes is to be interrupted
  and then later continued, such as when restarting an interrupted run
  or alternating between two or more problems, the user should save,
  following the return from the last lsodes call prior to the
  interruption, the contents of the call sequence variables and the
  internal common blocks, and later restore these values before the
  next lsodes call for that problem.  to save and restore the common
  blocks, use subroutine srcms (see part ii above).
  
  -----------------------------------------------------------------------------
  part iv.  optionally replaceable solver routines.
  
  below are descriptions of two routines in the lsodes package which
  relate to the measurement of errors.  either routine can be
  replaced by a user-supplied version, if desired.  however, since such
  a replacement may have a major impact on performance, it should be
  done only when absolutely necessary, and only with great caution.
  (note.. the means by which the package version of a routine is
  superseded by the user-s version may be system-dependent.)
  
  (a) ewset.
  the following subroutine is called just before each internal
  integration step, and sets the array of error weights, ewt, as
  described under itol/rtol/atol above..
      subroutine ewset (neq, itol, rtol, atol, ycur, ewt)
  where neq, itol, rtol, and atol are as in the lsodes call sequence,
  ycur contains the current dependent variable vector, and
  ewt is the array of weights set by ewset.
  
  if the user supplies this subroutine, it must return in ewt(i)
  (i = 1,...,neq) a positive quantity suitable for comparing errors
  in y(i) to.  the ewt array returned by ewset is passed to the
  vnorm routine (see below), and also used by lsodes in the computation
  of the optional output imxer, the diagonal jacobian approximation,
  and the increments for difference quotient jacobians.
  
  in the user-supplied version of ewset, it may be desirable to use
  the current values of derivatives of y.  derivatives up to order nq
  are available from the history array yh, described above under
  optional outputs.  in ewset, yh is identical to the ycur array,
  extended to nq + 1 columns with a column length of nyh and scale
  factors of h**j/factorial(j).  on the first call for the problem,
  given by nst = 0, nq is 1 and h is temporarily set to 1.0.
  the quantities nq, nyh, h, and nst can be obtained by including
  in ewset the statements..
      double precision h, rls
      common /ls0001/ rls(218),ils(39)
      nq = ils(35)
      nyh = ils(14)
      nst = ils(36)
      h = rls(212)
  thus, for example, the current value of dy/dt can be obtained as
  ycur(nyh+i)/h  (i=1,...,neq)  (and the division by h is
  unnecessary when nst = 0).
  
  (b) vnorm.
  the following is a real function routine which computes the weighted
  root-mean-square norm of a vector v..
      d = vnorm (n, v, w)
  where..
    n = the length of the vector,
    v = real array of length n containing the vector,
    w = real array of length n containing weights,
    d = sqrt( (1/n) * sum(v(i)*w(i))**2 ).
  vnorm is called with n = neq and with w(i) = 1.0/ewt(i), where
  ewt is as set by subroutine ewset.
  
  if the user supplies this function, it should return a non-negative
  value of vnorm suitable for use in the error control in lsodes.
  none of the arguments should be altered by vnorm.
  for example, a user-supplied vnorm routine might..
    -substitute a max-norm of (v(i)*w(i)) for the rms-norm, or
    -ignore some components of v in the norm, with the effect of
     suppressing the error control on those components of y.
  
  -----------------------------------------------------------------------------
  other routines in the lsodes package.
  
  in addition to subroutine lsodes, the lsodes package includes the
  following subroutines and function routines..
   iprep    acts as an iterface between lsodes and prep, and also does
            adjusting of work space pointers and work arrays.
   prep     is called by iprep to compute sparsity and do sparse matrix
            preprocessing if miter = 1 or 2.
   jgroup   is called by prep to compute groups of jacobian column
            indices for use when miter = 2.
   adjlr    adjusts the length of required sparse matrix work space.
            it is called by prep.
   cntnzu   is called by prep and counts the nonzero elements in the
            strict upper triangle of j + j-transpose, where j = df/dy.
   intdy    computes an interpolated value of the y vector at t = tout.
   stode    is the core integrator, which does one step of the
            integration and the associated error control.
   cfode    sets all method coefficients and test constants.
   prjs     computes and preprocesses the jacobian matrix j = df/dy
            and the newton iteration matrix p = i - h*l0*j.
   slss     manages solution of linear system in chord iteration.
   ewset    sets the error weight vector ewt before each step.
   vnorm    computes the weighted r.m.s. norm of a vector.
   srcms    is a user-callable routine to save and restore
            the contents of the internal common blocks.
   odrv     constructs a reordering of the rows and columns of
            a matrix by the minimum degree algorithm.  odrv is a
            driver routine which calls subroutines md, mdi, mdm,
            mdp, mdu, and sro.  see ref. 2 for details.  (the odrv
            module has been modified since ref. 2, however.)
   cdrv     performs reordering, symbolic factorization, numerical
            factorization, or linear system solution operations,
            depending on a path argument ipath.  cdrv is a
            driver routine which calls subroutines nroc, nsfc,
            nnfc, nnsc, and nntc.  see ref. 3 for details.
            lsodes uses cdrv to solve linear systems in which the
            coefficient matrix is  p = i - con*j, where i is the
            identity, con is a scalar, and j is an approximation to
            the jacobian df/dy.  because cdrv deals with rowwise
            sparsity descriptions, cdrv works with p-transpose, not p.
   d1mach   computes the unit roundoff in a machine-independent manner.
            It has been replaced in C by DBL_EPSILON.

  note..  vnorm is a function all the others are procedures.
  
  the intrinsic and external routines used by lsodes are..
  fabs, dmax1, dmin1, dfloat, max0, min0, mod, dsign, dsqrt, and write.
  
  a block data subprogram is also included with the package,
  for loading some of the variables in internal common.
  -------------------------------------------------------------------------- */

/* inclusions */

#include "lsodes.h"
#include "modiface.h"
#include "delays.h"
#include "yourcode.h"

/* globals */

struct {
  double conit, crate, el[13], elco[156], hold, rmax, tesco[36], ccmax, el0,
         h, hmin, hmxi, hu, rc, tn, uround;

  double con0, conmin, ccmxj, psmall, rbig, seth;

  long iplost, iesp, istatc, iys, iba, ibian, ibjan, ibjgp, ipian,
       ipjan, ipjgp, ipigp, ipr, ipc, ipic, ipisp, iprsp, ipa, lenyh,
       lenyhm, lenwk, lreq, lrat, lrest, lwmin, moss, msbj, nslj,
       ngp, nlu, nnz, nsp, nzl, nzu;

  long illin, init, lyh, lewt, lacor, lsavf, lwm, mxstep, mxhnil,
       nhnil, ntrep, nslast, nyh, ialth, ipup, lmax, meo, nqnyh, nslp,
       icf, ierpj, iersl, jcur, jstart, kflag, l, meth, miter, maxord, 
       maxcor, msbp, mxncf, n, nq, nst, nfe, nje, nqu;
} IGS;


/* ----------------------------------------------------------------------------
   d_sign

   The following is the definition of d_sign function used in
   lsodes_ and prep_ functions.
   d_sign returns |a1| if a2>=0 or -|a1| if a2<0, where a1 and a2
   are the two arguments passed to d_sign. However, their pointers
   are passed. ZGY, 11/30/93
*/
double d_sign(double *pa1, double *pa2)
{
  double a3;
  a3 = ((*pa1 >= 0) ? *pa1 : -*pa1);
  if (*pa2 >= 0) return a3;
  return -a3;
}


/* -------------------------------------------------------------------------- */
int lsodes_(long *neq, double *y,
            double *t, double *tout, long *itol, double *rtol,
            double *atol, long *itask, long *istate,
            long *iopt, double *rwork, long *lrw,
            long *iwork, long *liw, long *mf)
{

  /* Initialized data */

  static long mord[2] = {12, 5};
  static long mxstp0 = 500;
  static long mxhnl0 = 10;

  /* System generated locals */
  long i__1, i__2;
  double d__1, d__2;

  /* Local variables */
  long irem, imax, imul, ipgo, lyhd;
  long lyhn;
  double ewti, hmax, size;
  long ihit = 1;
  long i, j, iflag, leniw, imxer, lenrw, i1, i2, lrtem, lwtem, ncolm;
  double atoli, h0, rtoli, tcrit = 0, tdist, tnext, tolsf, w0;

  double rh;
  long ipflag;
  double tp;
  long lf0, lenyht, mf1;
  double big;
  long lia, lja, kgo;
  double ayi, hmx, tol, sum;

  long lenrat;
  lenrat = sizeof(double)/sizeof(long);
  if (lenrat < 1) lenrat = 1;

  /* Parameter adjustments */
  --iwork;
  --rwork;
  --atol;
  --rtol;
  --y;
  --neq;

  /* Function Body */
  /* ---------------------------------------------------------------------------
     block a.
     this code block is executed on every call.
     it tests istate and itask for legality and branches appropriately.
     if istate .gt. 1 but the flag init shows that initialization has
     not yet been done, an error return occurs.
     if istate = 1 and tout = t, jump to block g and return immediately.
  */

  if (*istate < 1 || *istate > 3) goto L601;

  if (*itask < 1 || *itask > 5) goto L602;

  if (*istate == 1) goto L10;

  if (IGS.init == 0) goto L603;

  if (*istate == 2) goto L200;

  goto L20;

L10:
  IGS.init = 0;
  if (*tout == *t) goto L430;

L20:
  IGS.ntrep = 0;

  /* ---------------------------------------------------------------------------
     block b.
     the next code block is executed for the initial call (istate = 1),
     or for a continuation call with parameter changes (istate = 3).
     it contains checking of all inputs and various initializations.
     if istate = 1, the final setting of work space pointers, the matrix
     preprocessing, and other initializations are done in block c.

     first check legality of the non-optional inputs neq, itol, iopt,
     mf, ml, and mu.
  */

  if (neq[1] <= 0) goto L604;

  if (*istate == 1) goto L25;

  if (neq[1] > IGS.n) goto L605;

L25:
  IGS.n = neq[1]; /* undeclared n, uses global */
  if (*itol < 1 || *itol > 4) goto L606;

  if (*iopt < 0 || *iopt > 1) goto L607;

  IGS.moss = *mf / 100;
  mf1 = *mf - IGS.moss * 100;
  IGS.meth = mf1 / 10;
  IGS.miter = mf1 - IGS.meth * 10;

  if (IGS.moss < 0 || IGS.moss > 2) goto L608;

  if (IGS.meth < 1 || IGS.meth > 2) goto L608;

  if (IGS.miter < 0 || IGS.miter > 3) goto L608;

  if (IGS.miter == 0 || IGS.miter == 3) IGS.moss = 0;

  /* next process and check the optional inputs. */

  if (*iopt == 1) goto L40;

  IGS.maxord = mord[IGS.meth - 1];
  IGS.mxstep = mxstp0;
  IGS.mxhnil = mxhnl0;
  if (*istate == 1) h0 = 0.0;

  IGS.hmxi = 0.0;
  IGS.hmin = 0.0;
  IGS.seth = 0.0;
  goto L60;

L40:
  IGS.maxord = iwork[5];
  if (IGS.maxord < 0) goto L611;

  if (IGS.maxord == 0) IGS.maxord = 100;

  /* Computing MIN */
  IGS.maxord = mymin(IGS.maxord,mord[IGS.meth - 1]);
  IGS.mxstep = iwork[6];

  if (IGS.mxstep < 0) goto L612;

  if (IGS.mxstep == 0) IGS.mxstep = mxstp0;

  IGS.mxhnil = iwork[7];

  if (IGS.mxhnil < 0) goto L613;

  if (IGS.mxhnil == 0) IGS.mxhnil = mxhnl0;

  if (*istate != 1) goto L50;

  h0 = rwork[5];
  if ((*tout - *t) * h0 < 0.) goto L614;

L50:
  hmax = rwork[6];
  if (hmax < 0.) goto L615;

  IGS.hmxi = 0.;
  if (hmax > 0.) IGS.hmxi = 1.0 / hmax;

  IGS.hmin = rwork[7];
  if (IGS.hmin < 0.) goto L616;

  IGS.seth = rwork[8];
  if (IGS.seth < 0.) goto L609;

L60:
  /* check rtol and atol for legality */
  rtoli = rtol[1];
  atoli = atol[1];
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    if (*itol >= 3) rtoli = rtol[i];

    if (*itol == 2 || *itol == 4) atoli = atol[i];

    if (rtoli < 0.) goto L619;

    if (atoli < 0.) goto L620;

  } /* end for i */

  /* ---------------------------------------------------------------------------
     compute required work array lengths, as far as possible, and test
     these against lrw and liw.  then set tentative pointers for work
     arrays.  pointers to rwork/iwork segments are named by prefixing l to
     the name of the segment.  e.g., the segment yh starts at rwork(lyh).

     segments of rwork (in order) are denoted  wm, yh, savf, ewt, acor.
     if miter = 1 or 2, the required length of the matrix work space wm
     is not yet known, and so a crude minimum value is used for the
     initial tests of lrw and liw, and yh is temporarily stored as far
     to the right in rwork as possible, to leave the maximum amount
     of space for wm for matrix preprocessing.  thus if miter = 1 or 2
     and moss .ne. 2, some of the segments of rwork are temporarily
     omitted, as they are not needed in the preprocessing.  these
     omitted segments are.. acor if istate = 1, ewt and acor if istate = 3
     and moss = 1, and savf, ewt, and acor if istate = 3 and moss = 0.
  */
  IGS.lrat = lenrat;
  if (*istate == 1) IGS.nyh = IGS.n;

  IGS.lwmin = 0;
  if (IGS.miter == 1)
    IGS.lwmin = (IGS.n << 2) + IGS.n * 10 / (double) IGS.lrat;

  if (IGS.miter == 2)
    IGS.lwmin = (IGS.n << 2) + IGS.n * 11 / (double) IGS.lrat;

  if (IGS.miter == 3) IGS.lwmin = IGS.n + 2;

  IGS.lenyh = (IGS.maxord + 1) * IGS.nyh;
  IGS.lrest = IGS.lenyh + IGS.n * 3;
  lenrw = IGS.lwmin + 20 + IGS.lrest;
  iwork[17] = lenrw;
  leniw = 30;
  if (IGS.moss == 0 && IGS.miter != 0 && IGS.miter != 3)
    leniw = leniw + IGS.n + 1;

  iwork[18] = leniw;
  if (lenrw > *lrw) goto L617;

  if (leniw > *liw) goto L618;

  lia = 31;
  if (IGS.moss == 0 && IGS.miter != 0 && IGS.miter != 3)
    leniw = leniw + iwork[lia + IGS.n] - 1;

  iwork[18] = leniw;
  if (leniw > *liw) goto L618;

  lja = lia + IGS.n + 1;
  lia = mymin(lia,*liw);
  lja = mymin(lja,*liw);
  IGS.lwm = 21;
  if (*istate == 1) IGS.nq = 1;

  /* Computing MIN */
  i__1 = IGS.nq + 1;
  i__2 = IGS.maxord + 2;
  ncolm = mymin(i__1,i__2);
  IGS.lenyhm = ncolm * IGS.nyh;
  lenyht = IGS.lenyh;
  if (IGS.miter == 1 || IGS.miter == 2)
    lenyht = IGS.lenyhm;

  imul = 2;
  if (*istate == 3) imul = IGS.moss;

  if (IGS.moss == 2) imul = 3;

  lrtem = lenyht + imul * IGS.n;
  lwtem = IGS.lwmin;
  if (IGS.miter == 1 || IGS.miter == 2)
    lwtem = *lrw - 20 - lrtem;

  IGS.lenwk = lwtem;
  lyhn = IGS.lwm + lwtem;
  IGS.lsavf = lyhn + lenyht;
  IGS.lewt = IGS.lsavf + IGS.n;
  IGS.lacor = IGS.lewt + IGS.n;
  IGS.istatc = *istate;

  if (*istate == 1) goto L100;

  /* ---------------------------------------------------------------------------
     istate = 3.  move yh to its new location.
     note that only the part of yh needed for the next step, namely
     min(nq+1,maxord+2) columns, is actually moved.
     a temporary error weight array ewt is loaded if moss = 2.
     sparse matrix processing is done in iprep/prep if miter = 1 or 2.
     if maxord was reduced below nq, then the pointers are finally set
     so that savf is identical to yh(*,maxord+2).
  */
  lyhd = IGS.lyh - lyhn;
  imax = lyhn - 1 + IGS.lenyhm;

  /* move yh.  branch for move right, no move, or move left. */
  if (lyhd < 0) goto L70;
  else
    if (lyhd == 0) goto L80;
    else goto L74;

L70:
  i__1 = imax;
  for (i = lyhn; i <= i__1; ++i) {
    j = imax + lyhn - i;
    rwork[j] = rwork[j + lyhd];
  }
  goto L80;

L74:
  i__1 = imax;
  for (i = lyhn; i <= i__1; ++i)
    rwork[i] = rwork[i + lyhd];

L80:
  IGS.lyh = lyhn;
  iwork[22] = IGS.lyh;
  if (IGS.miter == 0 || IGS.miter == 3) goto L92;

  if (IGS.moss != 2) goto L85;

  /* temporarily load ewt if miter = 1 or 2 and moss = 2. */
  ewset_(&(IGS.n), itol, &rtol[1], &atol[1], &rwork[IGS.lyh], &
         rwork[IGS.lewt]);

  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    if (rwork[i + IGS.lewt - 1] <= 0.) goto L621;
    rwork[i + IGS.lewt - 1] = 1. / rwork[i + IGS.lewt - 1];
  }

L85:
  /* iprep and prep do sparse matrix preprocessing if miter = 1 or 2. */
  IGS.lsavf = mymin(IGS.lsavf,*lrw);
  IGS.lewt = mymin(IGS.lewt,*lrw);
  IGS.lacor = mymin(IGS.lacor,*lrw);
  iprep_(&neq[1], &y[1], &rwork[1], &iwork[lia], &iwork[lja], &ipflag);
  lenrw = IGS.lwm - 1 + IGS.lenwk + IGS.lrest;
  iwork[17] = lenrw;
  if (ipflag != -1) iwork[23] = IGS.ipian;

  if (ipflag != -1) iwork[24] = IGS.ipjan;

  ipgo = -ipflag + 1;
  switch (ipgo) {
    case 1: goto L90;
    case 2: goto L628;
    case 3: goto L629;
    case 4: goto L630;
    case 5: goto L631;
    case 6: goto L632;
    case 7: goto L633;
  }

L90:
  iwork[22] = IGS.lyh;
  if (lenrw > *lrw) goto L617;

L92:
  /* set flag to signal parameter changes to stode. */
  IGS.jstart = -1;
  if (IGS.n == IGS.nyh) goto L200;

  /* neq was reduced. zero part of yh to avoid undefined references. */
  i1 = IGS.lyh + IGS.l * IGS.nyh;
  i2 = IGS.lyh + (IGS.maxord + 1) * IGS.nyh - 1;

  if (i1 > i2) goto L200;

  i__1 = i2;
  for (i = i1; i <= i__1; ++i) rwork[i] = 0.0;

  goto L200;

  /* ---------------------------------------------------------------------------
     block c.
     the next block is for the initial call only (istate = 1).
     it contains all remaining initializations, the initial call to f,
     the sparse matrix preprocessing (miter = 1 or 2), and the
     calculation of the initial step size.
     the error weights in ewt are inverted after being loaded.
  */
L100:
  IGS.lyh = lyhn;
  iwork[22] = IGS.lyh;
  IGS.tn = *t;
  IGS.nst = 0;
  IGS.h = 1.0;
  IGS.nnz = 0;
  IGS.ngp = 0;
  IGS.nzl = 0;
  IGS.nzu = 0;

  /* load the initial value vector in yh. */
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) rwork[i + IGS.lyh - 1] = y[i];

  /* initial call to f.  (lf0 points to yh(*,2).) */
  lf0 = IGS.lyh + IGS.nyh;
  CalcDeriv (&y[1], &rwork[lf0], t);

  IGS.nfe = 1;

  /* load and invert the ewt array.  (h is temporarily set to 1.0.) */
  ewset_(&(IGS.n), itol, &rtol[1], &atol[1], &rwork[IGS.lyh], &rwork[IGS.lewt]);

  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    if (rwork[i + IGS.lewt - 1] <= 0.) goto L621;
    rwork[i + IGS.lewt - 1] = 1. / rwork[i + IGS.lewt - 1];
  }

  if (IGS.miter == 0 || IGS.miter == 3) goto L120;

  /* iprep and prep do sparse matrix preprocessing if miter = 1 or 2. */
  IGS.lacor = mymin(IGS.lacor,*lrw);
  iprep_(&neq[1], &y[1], &rwork[1], &iwork[lia], &iwork[lja], &ipflag);
  lenrw = IGS.lwm - 1 + IGS.lenwk + IGS.lrest;
  iwork[17] = lenrw;
  if (ipflag != -1) iwork[23] = IGS.ipian;
  if (ipflag != -1) iwork[24] = IGS.ipjan;

  ipgo = -ipflag + 1;
  switch (ipgo) {
    case 1: goto L115;
    case 2: goto L628;
    case 3: goto L629;
    case 4: goto L630;
    case 5: goto L631;
    case 6: goto L632;
    case 7: goto L633;
  }

L115:
  iwork[22] = IGS.lyh;
  if (lenrw > *lrw) goto L617;

L120:
  /* check tcrit for legality (itask = 4 or 5). */
  if (*itask != 4 && *itask != 5) goto L125;

  tcrit = rwork[1];
  if ((tcrit - *tout) * (*tout - *t) < 0.) goto L625;
  if (h0 != 0. && (*t + h0 - tcrit) * h0 > 0.0) h0 = tcrit - *t;

L125:
  /* initialize all remaining parameters */
  IGS.uround = DBL_EPSILON;
  IGS.jstart = 0;
  if (IGS.miter != 0)
    rwork[IGS.lwm] = sqrt((double) IGS.uround);

  IGS.msbj = 50;
  IGS.nslj = 0;
  IGS.ccmxj = 0.2;
  IGS.psmall = IGS.uround * 1e3;
  IGS.rbig = 0.01 / IGS.psmall;
  IGS.nhnil = 0;
  IGS.nje = 0;
  IGS.nlu = 0;
  IGS.nslast = 0;
  IGS.hu = 0.0;
  IGS.nqu = 0;
  IGS.ccmax = 0.3;
  IGS.maxcor = 3;
  IGS.msbp = 20;
  IGS.mxncf = 10;

  /* ---------------------------------------------------------------------------
     the coding below computes the step size, h0, to be attempted on the
     first step, unless the user has supplied a value for this.
     first check that tout - t differs significantly from zero.
     a scalar tolerance quantity tol is computed, as max(rtol(i))
     if this is positive, or max(atol(i)/abs(y(i))) otherwise, adjusted
     so as to be between 100*uround and 1.0e-3.
     then the computed value h0 is given by..
                                         neq
     h0**2 = tol / ( w0**-2 + (1/neq) * sum ( f(i)/ywt(i) )**2  )
                                         1
     where   w0     = max ( abs(t), abs(tout) ),
             f(i)   = i-th component of initial value of f,
             ywt(i) = ewt(i)/tol  (a weight for y(i)).

     the sign of h0 is inferred from the initial values of tout and t.
  */
 lf0 = IGS.lyh + IGS.nyh;
 if (h0 != 0.) goto L180;

  tdist = fabs(*tout - *t);

  /* Computing MAX */
  w0 = mymax(fabs(*t), fabs(*tout));
  if (tdist < IGS.uround * 2.0 * w0) goto L622;

  tol = rtol[1];
  if (*itol <= 2) goto L140;

  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    /* Computing MAX */
    d__1 = tol, d__2 = rtol[i];
    tol = mymax(d__1,d__2);
  }

L140:
  if (tol > 0.) goto L160;

  atoli = atol[1];
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    if (*itol == 2 || *itol == 4) atoli = atol[i];
    ayi = fabs(y[i]);
    if (ayi != 0.0) {
      /* Computing MAX */
      d__1 = tol, d__2 = atoli / ayi;
      tol = mymax(d__1,d__2);
    } /* end if */
  } /* end for */

L160:
  /* Computing MAX */
  d__1 = tol;
  d__2 = IGS.uround * 100.;
  tol = mymax(d__1,d__2);
  tol = mymin(tol,0.001);
  sum = vnorm_(&(IGS.n), &rwork[lf0], &rwork[IGS.lewt]);

  /* Computing 2nd power */
  d__1 = sum;
  sum = 1.0 / (tol * w0 * w0) + tol * (d__1 * d__1);
  h0 = 1.0 / sqrt((double) sum);
  h0 = mymin(h0,tdist);
  d__1 = *tout - *t;
  h0 = d_sign(&h0, &d__1);

L180:
  /* adjust h0 if necessary to meet hmax bound */
  rh = fabs(h0) * IGS.hmxi;
  if (rh > 1.0) h0 /= rh;

  /* load h with h0 and scale yh(*,2) by h0 */
  IGS.h = h0;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i)
    rwork[i + lf0 - 1] = h0 * rwork[i + lf0 - 1];

  goto L270;

  /* ---------------------------------------------------------------------------
     block d.

     the next code block is for continuation calls only (istate = 2 or 3)
     and is to check stop conditions before taking a step.
  */
L200:
  IGS.nslast = IGS.nst;
  switch (*itask) {
    case 1:  goto L210;
    case 2:  goto L250;
    case 3:  goto L220;
    case 4:  goto L230;
    case 5:  goto L240;
  }

L210:
  if ((IGS.tn - *tout) * IGS.h < 0.0) goto L250;

  intdy_(tout, 0, &rwork[IGS.lyh], &(IGS.nyh), &y[1], &iflag);
  if (iflag != 0) goto L627;

  *t = *tout;
  goto L420;

L220:
  tp = IGS.tn - IGS.hu * (IGS.uround * 100.0 + 1.);
  if ((tp - *tout) * IGS.h > 0.0) goto L623;

  if ((IGS.tn - *tout) * IGS.h < 0.0) goto L250;

  goto L400;

L230:
  tcrit = rwork[1];
  if ((IGS.tn - tcrit) * IGS.h > 0.0) goto L624;

  if ((tcrit - *tout) * IGS.h < 0.0) goto L625;

  if ((IGS.tn - *tout) * IGS.h < 0.0) goto L245;

  intdy_(tout, 0, &rwork[IGS.lyh], &(IGS.nyh), &y[1], &iflag);

  if (iflag != 0) goto L627;

  *t = *tout;
  goto L420;

L240:
  tcrit = rwork[1];
  if ((IGS.tn - tcrit) * IGS.h > 0.0) goto L624;

L245:
  hmx = fabs(IGS.tn) + fabs(IGS.h);
  ihit = fabs(IGS.tn - tcrit) <= IGS.uround * 100.0 * hmx;

  if (ihit) goto L400;

  tnext = IGS.tn + IGS.h * (IGS.uround * 4.0 + 1.0);
  if ((tnext - tcrit) * IGS.h <= 0.0) goto L250;

  IGS.h = (tcrit - IGS.tn) * (1.0 - IGS.uround * 4.0);
  if (*istate == 2) IGS.jstart = -2;

  /* ---------------------------------------------------------------------------
     block e.

     the next block is normally executed for all calls and contains
     the call to the one-step core integrator stode.

     this is a looping point for the integration steps.

     first check for too many steps being taken, update ewt (if not at
     start of problem), check for too much accuracy being requested, and
     check for h below the roundoff level in t.
  */
L250:
  if (IGS.nst - IGS.nslast >= IGS.mxstep) goto L500;

  ewset_(&(IGS.n), itol, &rtol[1], &atol[1], &rwork[IGS.lyh], &rwork[IGS.lewt]);
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    if (rwork[i + IGS.lewt - 1] <= 0.) goto L510;
    rwork[i + IGS.lewt - 1] = 1. / rwork[i + IGS.lewt - 1];
  }

L270:
  tolsf = IGS.uround * vnorm_(&(IGS.n), &rwork[IGS.lyh], &rwork[IGS.lewt]);

  if (tolsf <= 1.) goto L280;

  tolsf *= 2.;
  if (IGS.nst == 0) goto L626;

  goto L520;

L280:
  if (IGS.tn + IGS.h != IGS.tn) goto L290;

  ++IGS.nhnil;
  if (IGS.nhnil > IGS.mxhnil) goto L290;

  printf ("lsodes: internal t (=%21.13f) and h (=%21.13f) are such that\n"
          "        t + h = t on the next step (h = step size).\n"
          "        Solver will continue anyway.\n",
          IGS.tn, IGS.h);

  if (IGS.nhnil < IGS.mxhnil) goto L290;

  printf ("lsodes: above warning has been issued %ld times.\n"
          "        It will not be issued again\n", IGS.mxhnil);

L290:
  /* call the StoreDelays and step_by_step routine for user-specified work.
     They are called before the stode routine to be on the actual
     differential system trajectory */
  if (bDelays) 
    StoreDelayed(IGS.tn);

  DoStep_by_Step();

  /* call stode(neq,y,yh,nyh,yh,ewt,savf,acor,wm,wm,f,jac,prjs,slss) */
  stode_(&neq[0], &y[0], &rwork[IGS.lyh], &(IGS.nyh),
         &rwork[IGS.lyh],   &rwork[IGS.lewt],
         &rwork[IGS.lsavf], &rwork[IGS.lacor],
         &rwork[IGS.lwm], (long *)&rwork[IGS.lwm]);

  kgo = 1 - IGS.kflag;
  switch (kgo) {
    case 1:  goto L300;
    case 2:  goto L530;
    case 3:  goto L540;
    case 4:  goto L550;
  }

  /* --------------------------------------------------------------------------
     block f.

     the following block handles the case of a successful return from the
     core integrator (kflag = 0).  test for stop conditions.
  */
L300:
  IGS.init = 1;
  switch (*itask) {
    case 1:  goto L310;
    case 2:  goto L400;
    case 3:  goto L330;
    case 4:  goto L340;
    case 5:  goto L350;
  }

L310:
  /* itask = 1.  if tout has been reached, interpolate */
  if ((IGS.tn - *tout) * IGS.h < 0.0) goto L250;
  intdy_(tout, 0, &rwork[IGS.lyh], &(IGS.nyh), &y[1], &iflag);
  *t = *tout;
  goto L420;

L330:
  /* itask = 3.  jump to exit if tout was reached */
  if ((IGS.tn - *tout) * IGS.h >= 0.0) goto L400;
  goto L250;

L340:
  /* itask = 4.  see if tout or tcrit was reached.
     adjust h if necessary */
  if ((IGS.tn - *tout) * IGS.h < 0.0) goto L345;

  intdy_(tout, 0, &rwork[IGS.lyh], &(IGS.nyh), &y[1], &iflag);

  *t = *tout;
  goto L420;

L345:
  hmx = fabs(IGS.tn) + fabs(IGS.h);
  ihit = fabs(IGS.tn - tcrit) <= IGS.uround * 100.0 * hmx;
  if (ihit) goto L400;

  tnext = IGS.tn + IGS.h * (IGS.uround * 4.0 + 1.0);
  if ((tnext - tcrit) * IGS.h <= 0.0) goto L250;

  IGS.h = (tcrit - IGS.tn) * (1.0 - IGS.uround * 4.0);
  IGS.jstart = -2;
  goto L250;

L350:
  /* itask = 5.  see if tcrit was reached and jump to exit */
  hmx = fabs(IGS.tn) + fabs(IGS.h);
  ihit = fabs(IGS.tn - tcrit) <= IGS.uround * 100.0 * hmx;

  /* ---------------------------------------------------------------------------
     block g.

     the following block handles all successful returns from lsodes.
     if itask .ne. 1, y is loaded from yh and t is set accordingly.
     istate is set to 2, the illegal input counter is zeroed, and the
     optional outputs are loaded into the work arrays before returning.
     if istate = 1 and tout = t, there is a return with no action taken,
     except that if this has happened repeatedly, the run is terminated.
  */
L400:
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) y[i] = rwork[i + IGS.lyh - 1];

  *t = IGS.tn;
  if (*itask != 4 && *itask != 5) goto L420;

  if (ihit) *t = tcrit;

L420:
  *istate = 2;
  IGS.illin = 0;
  rwork[11] = IGS.hu;
  rwork[12] = IGS.h;
  rwork[13] = IGS.tn;
  iwork[11] = IGS.nst;
  iwork[12] = IGS.nfe;
  iwork[13] = IGS.nje;
  iwork[14] = IGS.nqu;
  iwork[15] = IGS.nq;
  iwork[19] = IGS.nnz;
  iwork[20] = IGS.ngp;
  iwork[21] = IGS.nlu;
  iwork[25] = IGS.nzl;
  iwork[26] = IGS.nzu;
  return 0;

L430:
  ++IGS.ntrep;
  if (IGS.ntrep < 5) return 0;

  printf ("lsodes: repeated calls with istate = 1 and tout = t = %21.13f.\n",
          *t);
  goto L800;

  /* ---------------------------------------------------------------------------
     block h.

     the following block handles all unsuccessful returns other than
     those for illegal input.  first the error message routine is called.
     if there was an error test or convergence test failure, imxer is set.
     then y is loaded from yh, t is set to tn, and the illegal input
     counter illin is set to 0.  the optional outputs are loaded into
     the work arrays before returning.
  */
L500:
  /* the maximum number of steps was taken before reaching tout */
  printf ("lsodes: at t = %21.13f, mxstep (= %ld) steps\n"
          "        taken before reaching tout.\n",
          IGS.tn, IGS.mxstep);

  *istate = -1;
  goto L580;

L510:
  /* ewt(i) .le. 0.0 for some i (not at start of problem) */
  ewti = rwork[IGS.lewt + i - 1];
  printf ("lsodes: at t = %21.13f, ewt (%ld) has become %21.13f <= 0.\n",
          IGS.tn, i, ewti);

  *istate = -6;
  goto L580;

L520:
  /* too much accuracy requested for machine precision */
  printf ("lsodes: at t = %21.13f, too much accuracy requested\n"
          "        for precision of machine (tolsf = %21.13f).\n",
          IGS.tn, tolsf);

  rwork[14] = tolsf;
  *istate = -2;
  goto L580;

L530:
  /* kflag = -1.  error test failed repeatedly or with abs(h) = hmin */
  printf ("lsodes: at t = %21.13f and step size h = %21.13f, the error\n"
          "        test failed repeatedly or with abs(h) = hmin.\n",
          IGS.tn, IGS.h);

  *istate = -4;
  goto L560;

L540:
  /* kflag = -2.  convergence failed repeatedly or with abs(h) = hmin */
  printf ("lsodes: at t = %21.13f and step size h = %21.13f, the corrector\n"
          "        convergence failed repeatedly or with abs(h) = hmin.\n",
          IGS.tn, IGS.h);

  *istate = -5;
  goto L560;

L550:
  /* kflag = -3.  fatal error flag returned by prjs or slss (cdrv) */
  printf ("lsodes: at t = %21.13f and step size h = %21.13f, a fatal error flag\n"
          "        was returned by cdrv (by way of subroutine prjs or slss.\n",
          IGS.tn, IGS.h);

  *istate = -7;
  goto L580;

L560:
  /* compute imxer if relevant */
  big = 0.;
  imxer = 1;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    size = fabs(rwork[i + IGS.lacor - 1] * rwork[i + IGS.lewt - 1]);
    if (big >= size) goto L570;
    big = size;
    imxer = i;

L570: ;
  } /* end for i */

  iwork[16] = imxer;

L580:
  /* set y vector, t, illin, and optional outputs */
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) y[i] = rwork[i + IGS.lyh - 1];
  *t = IGS.tn;
  IGS.illin = 0;
  rwork[11] = IGS.hu;
  rwork[12] = IGS.h;
  rwork[13] = IGS.tn;
  iwork[11] = IGS.nst;
  iwork[12] = IGS.nfe;
  iwork[13] = IGS.nje;
  iwork[14] = IGS.nqu;
  iwork[15] = IGS.nq;
  iwork[19] = IGS.nnz;
  iwork[20] = IGS.ngp;
  iwork[21] = IGS.nlu;
  iwork[25] = IGS.nzl;
  iwork[26] = IGS.nzu;
  return 0;

  /* ---------------------------------------------------------------------------
     block i.

     the following block handles all error returns due to illegal input
     (istate = -3), as detected before calling the core integrator.
     first the error message routine is called.  then if there have been
     5 consecutive such returns just before this call to the solver,
     the run is halted.
  */
L601:
  printf ("lsodes: istate = %ld is illegal.\n", *istate);
  goto L700;

L602:
  printf ("lsodes: itask = %ld illegal.\n", *itask);
  goto L700;

L603:
  printf ("lsodes: istate is > 1 but lsodes is not initialized.\n");
  goto L700;

L604:
  printf ("lsodes: neq = %ld is lower than 1.\n", neq[1]);
  goto L700;

L605:
  printf ("lsodes: istate = 3 and neq increased from %ld to %ld.\n",
          IGS.n, neq[1]);
  goto L700;

L606:
  printf ("lsodes: itol = %ld is illegal.\n", *itol);
  goto L700;

L607:
  printf ("lsodes: iopt = %ld is illegal.\n", *iopt);
  goto L700;

L608:
  printf ("lsodes: mf = %ld is illegal.\n", *mf);
  goto L700;

L609:
  printf ("lsodes: seth = %21.13f is lower than 0.\n", IGS.seth);
  goto L700;

L611:
  printf ("lsodes: maxord = %ld is lower than 0.\n", IGS.maxord);
  goto L700;

L612:
  printf ("lsodes: mxstep = %ld is lower than 0.\n", IGS.mxstep);
  goto L700;

L613:
  printf ("lsodes: mxhnil = %ld is lower than 0.\n", IGS.mxhnil);
  goto L700;

L614:
  printf ("lsodes: tout = %21.13f is behind t = %21.13f\n", *tout, *t);
  printf ("        while integration direction is given by h0 = %21.13f.\n", h0);
  goto L700;

L615:
  printf ("lsodes: hmax = %21.13f, lower than 0.\n", hmax);
  goto L700;

L616:
  printf ("lsodes: hmin = %21.13f, lower than 0.\n", IGS.hmin);
  goto L700;

L617:
  printf ("lsodes: rwork length is insufficient to proceed,\n "
          "        length needed is at least %ld, exceeds liw (= %ld).\n",
          lenrw, *lrw);
  goto L700;

L618:
  printf ("lsodes: iwork length is insufficient to proceed,\n "
          "        length needed is at least %ld, exceeds liw (= %ld).\n",
          leniw, *liw);
  goto L700;

L619:
  printf ("lsodes: rtol[%ld] = %21.13f, lower than 0.\n", i, rtoli);
  goto L700;

L620:
  printf ("lsodes: atol[%ld] = %21.13f, lower than 0.\n", i, atoli);
  goto L700;

L621:
  ewti = rwork[IGS.lewt + i - 1];
  printf ("lsodes: ewt[%ld] = %21.13f, lower or equal to 0.\n", i, ewti);
  goto L700;

L622:
  printf ("lsodes: tout = %21.13f too close to t = %21.13f to start integration.\n",
          *tout, *t);
  goto L700;

L623:
  printf ("lsodes: itask = %ld and tout = %21.13f is behind tcur - hu (= %21.13f).\n",
          *itask, *tout, tp);
  goto L700;

L624:
  printf ("lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tcur (= %21.13f).\n",
          tcrit, IGS.tn);
  goto L700;

L625:
  printf ("lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tout = %21.13f.\n",
          tcrit, *tout);
  goto L700;

L626:
  printf ("lsodes: at start of the problem, too much accuracy requested\n"
          "        for precision of machine (tolsf = %21.13f).\n",
          tolsf);
  rwork[14] = tolsf;
  goto L700;

L627:
  printf ("lsodes: trouble from intdy. itask = %ld, tout = %21.13f.\n",
          *itask, *tout);
  goto L700;

L628:
  printf ("lsodes: rwork length insufficient (for subroutine prep),\n"
          "        length needed is at least %ld, exceeds lrw (= %ld).\n",
          lenrw, *lrw);
  goto L700;

L629:
  printf ("lsodes: rwork length insufficient (for subroutine jgroup),\n"
          "        length needed is at least %ld, exceeds lrw (= %ld).\n",
          lenrw, *lrw);
  goto L700;

L630:
  printf ("lsodes: rwork length insufficient (for subroutine odrv),\n"
          "        length needed is at least %ld, exceeds lrw (= %ld).\n",
          lenrw, *lrw);
  goto L700;

L631:
  imul = (IGS.iys - 1) / IGS.n;
  irem = IGS.iys - imul * IGS.n;
  printf ("lsodes: error from odrv in yale sparse matrix package,\n"
          "        at t = %21.13f, odrv returned error flag = %ld*neq + %ld:\n",
          IGS.tn, imul, irem);
  goto L700;

L632:
  printf ("lsodes: rwork length insufficient (for subroutine cdrv),\n"
          "        length needed is at least %ld, exceeds lrw (= %ld).\n",
          lenrw, *lrw);
  goto L700;

L633:
  printf ("lsodes: error from cdrv in yale sparse matrix package,\n");
  imul = (IGS.iys - 1) / IGS.n;
  irem = IGS.iys - imul * IGS.n;
  printf ("        at t =%21.13f, cdrv returned error flag = %ld*neq + %ld:\n",
          IGS.tn, imul, irem);
  if (imul == 2)
    printf ("        duplicate entry in sparsity structure descriptors.\n");
  if (imul == 3 || imul == 6)
    printf ("        insufficient storage for nsfc (called by cdrv).\n");

L700:
  if (IGS.illin == 5) goto L710;

  ++IGS.illin;
  *istate = -3;
  return 0;

L710:
  printf ("lsodes: repeated occurrences of illegal input.\n");

L800:
  printf ("lsodes: run aborted, apparent infinite loop.\n\n");
  abort();

  return 0;

} /* lsodes_ */


/* -----------------------------------------------------------------------------
   slss_

   this routine manages the solution of the linear system arising from
   a chord iteration.  it is called if miter .ne. 0.
   if miter is 1 or 2, it calls cdrv to accomplish this.
   if miter = 3 it updates the coefficient h*el0 in the diagonal
   matrix, and then computes the solution.
   communication with slss uses the following variables..
   wk    = real work space containing the inverse diagonal matrix if
           miter = 3 and the lu decomposition of the matrix otherwise.
           storage of matrix elements starts at wk(3).
           wk also contains the following matrix-related data..
           wk(1) = sqrt(uround) (not used here),
           wk(2) = hl0, the previous value of h*el0, used if miter = 3.

   iwk   = long work space for matrix-related data, assumed to
           be equivalenced to wk.  in addition, wk(iprsp) and iwk(ipisp)
           are assumed to have identical locations.
   x     = the right-hand side vector on input, and the solution vector
           on output, of length n.
   tem   = vector of work space of length n, not used in this version.
   iersl = output flag (in common).
           iersl = 0  if no trouble occurred.
           iersl = -1 if cdrv returned an error flag (miter = 1 or 2).
                      this should never occur and is considered fatal.
           iersl = 1  if a singular matrix arose with miter = 3.
   this routine also uses other variables in common.
*/
int slss_(double *wk, long *iwk, double *x, double *tem)
{
  /* System generated locals */
  long i__1;

  /* Local variables */
  long i;
  double r, di, hl0, phl0;

  /* Parameter adjustments */
  --tem;
  --x;
  --iwk;
  --wk;

  /* Function Body */
  IGS.iersl = 0;
  switch (IGS.miter) {
    case 1:  goto L100;
    case 2:  goto L100;
    case 3:  goto L300;
  }

L100:
  cdrv_(&(IGS.n), &iwk[IGS.ipr], &iwk[IGS.ipc], &iwk[IGS.ipic], &iwk[IGS.ipian],
        &iwk[IGS.ipjan], &wk[IGS.ipa], &x[1], &x[1], &(IGS.nsp), &iwk[IGS.ipisp],
        &wk[IGS.iprsp], &(IGS.iesp), 4, &(IGS.iersl));

  if (IGS.iersl != 0) IGS.iersl = -1;

  return 0;

L300:
  phl0 = wk[2];
  hl0 = IGS.h * IGS.el0;
  wk[2] = hl0;
  if (hl0 == phl0) goto L330;
  r = hl0 / phl0;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) {
    di = 1.0 - r * (1.0 - 1.0 / wk[i + 2]);
    if (fabs(di) == 0.) goto L390;
    wk[i + 2] = 1.0 / di;
  }

L330:
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) x[i] = wk[i + 2] * x[i];

  return 0;

L390:
  IGS.iersl = 1;

  return 0;

} /* slss_ */


/* -----------------------------------------------------------------------------
   prjs_

   prjs is called to compute and process the matrix
   p = i - h*el(1)*j , where j is an approximation to the jacobian.
   j is computed by columns, either by the user-supplied routine jac
   if miter = 1, or by finite differencing if miter = 2.
   if miter = 3, a diagonal approximation to j is used.
   if miter = 1 or 2, and if the existing value of the jacobian
   (as contained in p) is considered acceptable, then a new value of
   p is reconstructed from the old value.  in any case, when miter
   is 1 or 2, the p matrix is subjected to lu decomposition in cdrv.
   p and its lu decomposition are stored (separately) in wk.

   in addition to variables described previously, communication
   with prjs uses the following..
   y     = array containing predicted values on entry.
   ftem  = work array of length n (acor in stode).
   savf  = array containing f evaluated at predicted y.
   wk    = real work space for matrices.  on output it contains the
           inverse diagonal matrix if miter = 3, and p and its sparse
           lu decomposition if miter is 1 or 2.
           storage of matrix elements starts at wk(3).
           wk also contains the following matrix-related data..
           wk(1) = sqrt(uround), used in numerical jacobian increments.
           wk(2) = h*el0, saved for later use if miter = 3.
   iwk   = long work space for matrix-related data, assumed to
           be equivalenced to wk.  in addition, wk(iprsp) and iwk(ipisp)
           are assumed to have identical locations.
   el0   = el(1) (input).
   ierpj = output error flag (in common).
         = 0 if no error.
         = 1  if zero pivot found in cdrv.
         = 2  if a singular matrix arose with miter = 3.
         = -1 if insufficient storage for cdrv (should not occur here).
         = -2 if other error found in cdrv (should not occur here).
   jcur  = output flag = 1 to indicate that the jacobian matrix
           (or approximation) is now current.
   this routine also uses other variables in common.
*/
int prjs_(long *neq, double *y, double *yh,
          long *nyh, double *ewt, double *ftem, double *savf,
          double *wk, long *iwk)
{
  /* System generated locals */
  long yh_dim1, yh_offset, i__1, i__2, i__3;
  double d__1, d__2, d__3;

  /* Local variables */
  long jmin, jmax, imul, kmax, kmin;
  double rcon;
  double srur;
  long i, j, k;
  double r, rcont;
  double r0, di;
  long jj, ng;
  double hl0, fac, con, pij;
  long jok;

  /* Parameter adjustments */
  --iwk;
  --wk;
  --savf;
  --ftem;
  --ewt;
  yh_dim1 = *nyh;
  yh_offset = yh_dim1 + 1;
  yh -= yh_offset;
  --y;
  --neq;

  /* Function Body */

  hl0 = IGS.h * IGS.el0;
  con = -hl0;

  if (IGS.miter == 3) {

    /* if miter = 3, construct a diagonal approximation to j and p. */
    IGS.jcur = 1;
    ++IGS.nje;
    wk[2] = hl0;
    IGS.ierpj = 0;
    r = IGS.el0 * 0.1;

    for (i = 1; i <= IGS.n; ++i)
      y[i] += r * (IGS.h * savf[i] - yh[i + (yh_dim1 << 1)]);

    CalcDeriv (&y[1], &wk[3], &(IGS.tn));

    ++IGS.nfe;
    for (i = 1; i <= IGS.n; ++i) {
      r0 = IGS.h * savf[i] - yh[i + (yh_dim1 << 1)];
      di = r0 * 0.1 - IGS.h * (wk[i + 2] - savf[i]);
      wk[i + 2] = 1.0;

      if ( fabs(r0) >= (IGS.uround / ewt[i])) {
        if (fabs(di) == 0.0) {
          IGS.ierpj = 2;
          return 0;
        }
        wk[i + 2] = r0 * 0.1 / di;

      } /* end if */

    } /* end for */

    return 0;

  } /* end if miter == 3 */

  /* see whether j should be reevaluated (jok = 0) or not (jok = 1). */
  jok = 1;
  if (IGS.nst == 0 || IGS.nst >= IGS.nslj + IGS.msbj)
    jok = 0;

  if ((IGS.icf == 1) && (fabs(IGS.rc - 1) < IGS.ccmxj)) jok = 0;

  if (IGS.icf == 2) jok = 0;

  if (jok == 1) goto L250;

L20:
  /* miter = 1 or 2, and the jacobian is to be reevaluated. */
  IGS.jcur = 1;
  ++IGS.nje;
  IGS.nslj = IGS.nst;
  IGS.iplost = 0;
  IGS.conmin = fabs(con);
  switch (IGS.miter) {
    case 1:  goto L100;
    case 2:  goto L200;
  }

L100:
  /* if miter = 1, call jac, multiply by scalar, and add identity. */
  kmin = iwk[IGS.ipian];
  i__1 = IGS.n;
  for (j = 1; j <= i__1; ++j) {
    kmax = iwk[IGS.ipian + j] - 1;

    for (i = 1; i <= IGS.n; ++i)
      ftem[i] = 0.0;

    CalcJacob (&(IGS.tn), &y[1], j, &ftem[1]);

    i__2 = kmax;
    for (k = kmin; k <= i__2; ++k) {
      i = iwk[IGS.ibjan + k];
      wk[IGS.iba + k] = ftem[i] * con;
      if (i == j) wk[IGS.iba + k] += 1.0;
    } /* end for */

    kmin = kmax + 1;
  } /* end for j */

  goto L290;

L200:
  /* if miter = 2, make ngp calls to f to approximate j and p */
  fac = vnorm_(&(IGS.n), &savf[1], &ewt[1]);
  r0 = fabs(IGS.h) * 1e3 * IGS.uround * (double) IGS.n * fac;
  if (r0 == 0.) r0 = 1.0;

  srur = wk[1];
  jmin = iwk[IGS.ipigp];
  i__1 = IGS.ngp;
  for (ng = 1; ng <= i__1; ++ng) {
    jmax = iwk[IGS.ipigp + ng] - 1;
    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {
      jj = iwk[IGS.ibjgp + j];

      /* Computing MAX */
      d__2 = srur * fabs(y[jj]), d__3 = r0 / ewt[jj];
      r = mymax(d__2,d__3);
      y[jj] += r;
    }

    CalcDeriv (&y[1], &ftem[1], &(IGS.tn));

    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {
      jj = iwk[IGS.ibjgp + j];
      y[jj] = yh[jj + yh_dim1];

      /* Computing MAX */
      d__2 = srur * fabs(y[jj]), d__3 = r0 / ewt[jj];
      r = mymax(d__2,d__3);
      fac = -hl0 / r;
      kmin = iwk[IGS.ibian + jj];
      kmax = iwk[IGS.ibian + jj + 1] - 1;
      i__3 = kmax;
      for (k = kmin; k <= i__3; ++k) {
        i = iwk[IGS.ibjan + k];
        wk[IGS.iba + k] = (ftem[i] - savf[i]) * fac;
        if (i == jj) wk[IGS.iba + k] += 1.0;
      } /* end for */
    } /* end for */

    jmin = jmax + 1;

  }
  IGS.nfe += IGS.ngp;
  goto L290;

L250:
  /* if jok = 1, reconstruct new p from old p */
  IGS.jcur = 0;
  rcon = con / IGS.con0;
  rcont = fabs(con) / IGS.conmin;
  if (rcont > IGS.rbig && IGS.iplost == 1) goto L20;

  kmin = iwk[IGS.ipian];
  i__1 = IGS.n;
  for (j = 1; j <= i__1; ++j) {
    kmax = iwk[IGS.ipian + j] - 1;
    i__2 = kmax;
    for (k = kmin; k <= i__2; ++k) {
      i = iwk[IGS.ibjan + k];
      pij = wk[IGS.iba + k];
      if (i != j) goto L260;
      pij += -1.;
      if (fabs(pij) >= IGS.psmall) goto L260;
      IGS.iplost = 1;

      /* Computing MIN */
      d__1 = fabs(IGS.con0);
      IGS.conmin = mymin(d__1,IGS.conmin);

L260:
      pij *= rcon;
      if (i == j) pij += 1.0;

      wk[IGS.iba + k] = pij;

    } /* end for k */
    kmin = kmax + 1;

  } /* end for j */

L290:
  /* do numerical factorization of p matrix */
  ++IGS.nlu;
  IGS.con0 = con;
  IGS.ierpj = 0;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i) ftem[i] = 0.0;

  cdrv_(&(IGS.n), &iwk[IGS.ipr], &iwk[IGS.ipc],
        &iwk[IGS.ipic], &iwk[IGS.ipian], &iwk[IGS.ipjan],
        &wk[IGS.ipa], &ftem[1], &ftem[1], &(IGS.nsp),
        &iwk[IGS.ipisp], &wk[IGS.iprsp], &(IGS.iesp), 2,
        &(IGS.iys));

  if (IGS.iys == 0) return 0;

  imul = (IGS.iys - 1) / IGS.n;
  IGS.ierpj = -2;
  if (imul == 8) IGS.ierpj = 1;

  if (imul == 10) IGS.ierpj = -1;

  return 0;

} /* end of prjs_ */


/* ----------------------------------------------------------------------------
   stode

   Performs one step of the integration of an initial value
   problem for a system of ordinary differential equations.
   note.. stode is independent of the value of the iteration method
   indicator miter, when this is .ne. 0, and hence is independent
   of the type of chord method used, or the jacobian structure.
   communication with stode is done with the following variables..

   neq    = long array containing problem size in neq(1), and
            passed as the neq argument in all calls to f and jac.
   y      = an array of length .ge. n used as the y argument in
            all calls to f and jac.
   yh     = an nyh by lmax array containing the dependent variables
            and their approximate scaled derivatives, where
            lmax = maxord + 1.  yh(i,j+1) contains the approximate
            j-th derivative of y(i), scaled by h**j/factorial(j)
            (j = 0,1,...,nq).  on entry for the first step, the first
            two columns of yh must be set from the initial values.
   nyh    = a constant long .ge. n, the first dimension of yh.
   yh1    = a one-dimensional array occupying the same space as yh.
   ewt    = an array of length n containing multiplicative weights
            for local error measurements.  local errors in y(i) are
            compared to 1.0/ewt(i) in various error tests.
   savf   = an array of working storage, of length n.
            also used for input of yh(*,maxord+2) when jstart = -1
            and maxord .lt. the current order nq.
   acor   = a work array of length n, used for the accumulated
            corrections.  on a successful return, acor(i) contains
            the estimated one-step local error in y(i).
   wm,iwm = real and long work arrays associated with matrix
            operations in chord iteration (miter .ne. 0).
   pjac   = name of routine to evaluate and preprocess jacobian matrix
            and p = i - h*el0*jac, if a chord method is being used.
   slvs   = name of routine to solve linear system in chord iteration.
   ccmax  = maximum relative change in h*el0 before pjac is called.
   h      = the step size to be attempted on the next step.
            h is altered by the error control algorithm during the
            problem.  h can be either positive or negative, but its
            sign must remain constant throughout the problem.
   hmin   = the minimum absolute value of the step size h to be used.
   hmxi   = inverse of the maximum absolute value of h to be used.
            hmxi = 0.0 is allowed and corresponds to an infinite hmax.
            hmin and hmxi may be changed at any time, but will not
            take effect until the next change of h is considered.
   tn     = the independent variable. tn is updated on each step taken.
   jstart = an long used for input only, with the following
            values and meanings..
                 0  perform the first step.
             .gt.0  take a new step continuing from the last.
                -1  take the next step with a new value of h, maxord,
                      n, meth, miter, and/or matrix parameters.
                -2  take the next step with a new value of h,
                      but with other inputs unchanged.
            on return, jstart is set to 1 to facilitate continuation.
   kflag  = a completion code with the following meanings..
                 0  the step was succesful.
                -1  the requested error could not be achieved.
                -2  corrector convergence could not be achieved.
                -3  fatal error in pjac or slvs.
            a return with kflag = -1 or -2 means either
            abs(h) = hmin or 10 consecutive failures occurred.
            on a return with kflag negative, the values of tn and
            the yh array are as of the beginning of the last
            step, and h is the last step size attempted.
   maxord = the maximum order of integration method to be allowed.
   maxcor = the maximum number of corrector iterations allowed.
   msbp   = maximum number of steps between pjac calls (miter .gt. 0).
   mxncf  = maximum number of convergence failures allowed.
   meth/miter = the method flags.  see description in driver.
   n      = the number of first-order differential equations.
*/

int stode_(long *neq, double *y, double *yh,
           long *nyh, double *yh1, double *ewt, double *savf,
           double *acor, double *wm, long *iwm)
{
  /* System generated locals */
  long yh_dim1, yh_offset, i__1, i__2;
  double d__1, d__2, d__3;

  /* Local variables */
  double dcon, delp, rhdn, exdn;
  long iret;
  double told, rhsm;
  long newq;
  double exsm, rhup, exup;
  long i, j, m;
  double r;
  long i1,iredo = 0;
  long jb;
  double del, ddn, rh = 0.0;
  long ncf;
  double dup, dsm = 0.0;

  /* Parameter adjustments */
  --iwm;
  --wm;
  --acor;
  --savf;
  --ewt;
  --yh1;
  yh_dim1 = *nyh;
  yh_offset = yh_dim1 + 1;
  yh -= yh_offset;

  /* Function Body */
  IGS.kflag = 0;
  told = IGS.tn;
  ncf = 0;
  IGS.ierpj = 0;
  IGS.iersl = 0;
  IGS.jcur = 0;
  IGS.icf = 0;
  delp = 0.;
  if (IGS.jstart > 0) goto L200;
  if (IGS.jstart == -1) goto L100;
  if (IGS.jstart == -2) goto L160;

  /* --------------------------------------------------------------------------
     on the first call, the order is set to 1, and other variables are
     initialized. rmax is the maximum ratio by which h can be increased
     in a single step.  it is initially 1.e4 to compensate for the small
     initial h, but then is normally equal to 10.  if a failure
     occurs (in corrector convergence or error test), rmax is set at 2
     for the next increase.
  */
  IGS.lmax = IGS.maxord + 1;
  IGS.nq = 1;
  IGS.l = 2;
  IGS.ialth = 2;
  IGS.rmax = 1e4;
  IGS.rc = 0.;
  IGS.el0 = 1.;
  IGS.crate = .7;
  IGS.hold = IGS.h;
  IGS.meo = IGS.meth;
  IGS.nslp = 0;
  IGS.ipup = IGS.miter;
  iret = 3;
  goto L140;

L100:
  /* --------------------------------------------------------------------------
     the following block handles preliminaries needed when jstart = -1.
     ipup is set to miter to force a matrix update.
     if an order increase is about to be considered (ialth = 1),
     ialth is reset to 2 to postpone consideration one more step.
     if the caller has changed meth, cfode is called to reset
     the coefficients of the method.
     if the caller has changed maxord to a value less than the current
     order nq, nq is reduced to maxord, and a new h chosen accordingly.
     if h is to be changed, yh must be rescaled.
     if h or meth is being changed, ialth is reset to l = nq + 1
     to prevent further changes in h for that many steps.
  */
  IGS.ipup = IGS.miter;
  IGS.lmax = IGS.maxord + 1;
  if (IGS.ialth == 1) IGS.ialth = 2;

  if (IGS.meth == IGS.meo) goto L110;

  cfode_(&(IGS.meth), IGS.elco, IGS.tesco);

  IGS.meo = IGS.meth;
  if (IGS.nq > IGS.maxord) goto L120;

  IGS.ialth = IGS.l;
  iret = 1;
  goto L150;

L110:
  if (IGS.nq <= IGS.maxord) goto L160;

L120:
  IGS.nq = IGS.maxord;
  IGS.l = IGS.lmax;
  i__1 = IGS.l;
  for (i = 0; i < i__1; ++i)
    IGS.el[i] = IGS.elco[i + IGS.nq * 13 - 13];

  IGS.nqnyh = IGS.nq * *nyh;
  IGS.rc = IGS.rc * IGS.el[0] / IGS.el0;
  IGS.el0 = IGS.el[0];
  IGS.conit = 0.5 / (double) (IGS.nq + 2);
  ddn = vnorm_(&(IGS.n), &savf[1], &ewt[1]) /
               IGS.tesco[IGS.l * 3 - 3];

  exdn = 1.0 / (double) IGS.l;
  rhdn = 1.0 / (pow(ddn, exdn) * 1.3 + 1.3e-6);
  rh = mymin(rhdn,1.0);
  iredo = 3;
  if (IGS.h == IGS.hold) goto L170;

  /* Computing MIN */
  d__3 = fabs(IGS.h / IGS.hold);
  rh = mymin(rh,d__3);
  IGS.h = IGS.hold;
  goto L175;

L140:
  /* --------------------------------------------------------------------------
     cfode is called to get all the integration coefficients for the
     current meth.  then the el vector and related constants are reset
     whenever the order nq is changed, or at the start of the problem.
  */
  cfode_(&(IGS.meth), IGS.elco, IGS.tesco);

L150:
  i__1 = IGS.l;
  for (i = 1; i <= i__1; ++i)
    IGS.el[i - 1] = IGS.elco[i + IGS.nq * 13 - 14];

  IGS.nqnyh = IGS.nq * *nyh;
  IGS.rc = IGS.rc * IGS.el[0] / IGS.el0;
  IGS.el0 = IGS.el[0];
  IGS.conit = 0.5 / (double) (IGS.nq + 2);
  switch (iret) {
    case 1:  goto L160;
    case 2:  goto L170;
    case 3:  goto L200;
  }

L160:
  /* ---------------------------------------------------------------------------
     if h is being changed, the h ratio rh is checked against
     rmax, hmin, and hmxi, and the yh array rescaled.  ialth is set to
     l = nq + 1 to prevent a change of h for that many steps, unless
     forced by a convergence or error test failure.
   */
  if (IGS.h == IGS.hold) goto L200;

  rh = IGS.h / IGS.hold;
  IGS.h = IGS.hold;
  iredo = 3;
  goto L175;

L170:
  /* Computing MAX */
  d__2 = IGS.hmin / fabs(IGS.h);
  rh = mymax(rh,d__2);

L175:
  rh = mymin(rh,IGS.rmax);

  /* Computing MAX */
  d__2 = fabs(IGS.h) * IGS.hmxi * rh;
  rh /= mymax(1.0,d__2);
  r = 1.0;
  i__1 = IGS.l;
  for (j = 2; j <= i__1; ++j) {
    r *= rh;
    for (i = 1; i <= IGS.n; ++i)
      yh[i + j * yh_dim1] *= r;
  }

  IGS.h *= rh;
  IGS.rc *= rh;
  IGS.ialth = IGS.l;
  if (iredo == 0) goto L690;

L200:
  /* --------------------------------------------------------------------------
     this section computes the predicted values by effectively
     multiplying the yh array by the pascal triangle matrix.
     rc is the ratio of new to old values of the coefficient  h*el(1).
     when rc differs from 1 by more than ccmax, ipup is set to miter
     to force pjac to be called, if a jacobian is involved.
     in any case, pjac is called at least every msbp steps.
  */
  if (fabs(IGS.rc - 1) > IGS.ccmax)
    IGS.ipup = IGS.miter;

  if (IGS.nst >= IGS.nslp + IGS.msbp)
    IGS.ipup = IGS.miter;

  IGS.tn += IGS.h;
  i1 = IGS.nqnyh + 1;
  i__2 = IGS.nq;
  for (jb = 1; jb <= i__2; ++jb) {
    i1 -= *nyh;
    i__1 = IGS.nqnyh;
    for (i = i1; i <= i__1; ++i) yh1[i] += yh1[i + *nyh];
  }

L220:
  /* --------------------------------------------------------------------------
     up to maxcor corrector iterations are taken. A convergence test is
     made on the r.m.s. norm of each correction, weighted by the error
     weight vector ewt.  the sum of the corrections is accumulated in the
     vector acor(i).  the yh array is not altered in the corrector loop.
  */
  m = 0;
  i__2 = IGS.n;
  for (i = 1; i <= i__2; ++i) y[i] = yh[i + yh_dim1];

  CalcDeriv (&y[1], &savf[1], &(IGS.tn));

  ++IGS.nfe;
  if (IGS.ipup <= 0) goto L250;

  /* --------------------------------------------------------------------------
     if indicated, the matrix p = i - h*el(1)*j is reevaluated and
     preprocessed before starting the corrector iteration.  ipup is set
     to 0 as an indicator that this has been done.
  */

  prjs_ (&neq[1], &y[1], &yh[yh_offset], nyh, &ewt[1], &acor[1], &savf[1],
         &wm[1], &iwm[1]);

  IGS.ipup = 0;
  IGS.rc = 1.0;
  IGS.nslp = IGS.nst;
  IGS.crate = 0.7;
  if (IGS.ierpj != 0) goto L430;

L250:
  for (i = 1; i <= IGS.n; ++i)
    acor[i] = 0.;

L270:
  if (IGS.miter != 0) goto L350;

  /* --------------------------------------------------------------------------
     in the case of functional iteration, update y directly from
     the result of the last function evaluation.
  */
  i__2 = IGS.n;
  for (i = 1; i <= i__2; ++i) {
    savf[i] = IGS.h * savf[i] - yh[i + (yh_dim1 << 1)];
    y[i] = savf[i] - acor[i];
  }

  del = vnorm_(&(IGS.n), &y[1], &ewt[1]);

  for (i = 1; i <= IGS.n; ++i) {
    y[i] = yh[i + yh_dim1] + IGS.el[0] * savf[i];
    acor[i] = savf[i];
  }
  goto L400;

L350:
  /* --------------------------------------------------------------------------
     in the case of the chord method, compute the corrector error,
     and solve the linear system with that as right-hand side and
     p as coefficient matrix.
  */
  i__2 = IGS.n;
  for (i = 1; i <= i__2; ++i)
    y[i] = IGS.h * savf[i] - (yh[i + (yh_dim1 << 1)] + acor[i]);

  slss_ (&wm[1], &iwm[1], &y[1], &savf[1]);

  if (IGS.iersl < 0) goto L430;

  if (IGS.iersl > 0) goto L410;

  del = vnorm_(&(IGS.n), &y[1], &ewt[1]);

  i__2 = IGS.n;
  for (i = 1; i <= i__2; ++i) {
    acor[i] += y[i];
    y[i] = yh[i + yh_dim1] + IGS.el[0] * acor[i];
  }

L400:
  /* --------------------------------------------------------------------------
     test for convergence.  if m.gt.0, an estimate of the convergence
     rate constant is stored in crate, and this is used in the test.
  */
  if (m != 0) {
    /* Computing MAX */
    d__1 = IGS.crate * 0.2;
    d__2 = del / delp;
    IGS.crate = mymax(d__1,d__2);
  }

  /* Computing MIN */
  d__2 = IGS.crate * 1.5;
  dcon = del * mymin(1.0,d__2) /
         (IGS.tesco[IGS.nq * 3 - 2] * IGS.conit);

  if (dcon <= 1.) goto L450;

  ++m;
  if (m == IGS.maxcor) goto L410;

  if (m >= 2 && del > delp * 2.0) goto L410;

  delp = del;

  CalcDeriv (&y[1], &savf[1], &(IGS.tn));

  ++IGS.nfe;
  goto L270;

L410:
  /* --------------------------------------------------------------------------
     the corrector iteration failed to converge.
     if miter .ne. 0 and the jacobian is out of date, pjac is called for
     the next try.  otherwise the yh array is retracted to its values
     before prediction, and h is reduced, if possible.  if h cannot be
     reduced or mxncf failures have occurred, exit with kflag = -2.
  */
  if (IGS.miter == 0 || IGS.jcur == 1) goto L430;

  IGS.icf = 1;
  IGS.ipup = IGS.miter;
  goto L220;

L430:
  IGS.icf = 2;
  ++ncf;
  IGS.rmax = 2.0;
  IGS.tn = told;
  i1 = IGS.nqnyh + 1;

  for (jb = 1; jb <= IGS.nq; ++jb) {
    i1 -= *nyh;
    for (i = i1; i <= IGS.nqnyh; ++i) 
      yh1[i] -= yh1[i + *nyh];
  }

  if (IGS.ierpj < 0 || IGS.iersl < 0) goto L680;

  if (fabs(IGS.h) <= IGS.hmin * 1.00001) goto L670;

  if (ncf == IGS.mxncf) goto L670;

  rh = 0.25;
  IGS.ipup = IGS.miter;
  iredo = 1;
  goto L170;

L450:
  /* --------------------------------------------------------------------------
     the corrector has converged.  jcur is set to 0
     to signal that the jacobian involved may need updating later.
     the local error test is made and control passes to statement 500
     if it fails.
   */
  IGS.jcur = 0;
  if (m == 0)
    dsm = del / IGS.tesco[IGS.nq * 3 - 2];

  if (m > 0)
    dsm = vnorm_(&(IGS.n), &acor[1], &ewt[1]) / IGS.tesco[IGS.nq * 3 - 2];

  if (dsm > 1.0) goto L500;

  /* --------------------------------------------------------------------------
     after a successful step, update the yh array.
     consider changing h if ialth = 1.  otherwise decrease ialth by 1.
     if ialth is then 1 and nq .lt. maxord, then acor is saved for
     use in a possible order increase on the next step.
     if a change in h is considered, an increase or decrease in order
     by one is considered also.  a change in h is made only if it is by a

     factor of at least 1.1.  if not, ialth is set to 3 to prevent
     testing for that many steps.
  */
  IGS.kflag = 0;
  iredo = 0;
  ++IGS.nst;
  IGS.hu = IGS.h;
  IGS.nqu = IGS.nq;
  for (j = 1; j <= IGS.l; ++j) {
    for (i = 1; i <= IGS.n; ++i)
      yh[i + j * yh_dim1] += IGS.el[j - 1] * acor[i];
  }

  --IGS.ialth;
  if (IGS.ialth == 0) goto L520;
  if (IGS.ialth > 1)  goto L700;
  if (IGS.l == IGS.lmax)  goto L700;

  for (i = 1; i <= IGS.n; ++i)
    yh[i + IGS.lmax * yh_dim1] = acor[i];

  goto L700;

L500:
  /* --------------------------------------------------------------------------
     the error test failed.  kflag keeps track of multiple failures.
     restore tn and the yh array to their previous values, and prepare
     to try the step again.  compute the optimum step size for this or
     one lower order.  after 2 or more failures, h is forced to decrease
     by a factor of 0.2 or less.
  */
  --IGS.kflag;
  IGS.tn = told;
  i1 = IGS.nqnyh + 1;
  i__1 = IGS.nq;
  for (jb = 1; jb <= i__1; ++jb) {
    i1 -= *nyh;
    i__2 = IGS.nqnyh;
    for (i = i1; i <= i__2; ++i) yh1[i] -= yh1[i + *nyh];
  }

  IGS.rmax = 2.0;
  if (fabs(IGS.h) <= IGS.hmin * 1.00001) goto L660;

  if (IGS.kflag <= -3) goto L640;

  iredo = 2;
  rhup = 0.0;
  goto L540;

L520:
  /* --------------------------------------------------------------------------
     regardless of the success or failure of the step, factors
     rhdn, rhsm, and rhup are computed, by which h could be multiplied
     at order nq - 1, order nq, or order nq + 1, respectively.
     in the case of failure, rhup = 0.0 to avoid an order increase.
     the largest of these is determined and the new order chosen
     accordingly.  if the order is to be increased, we compute one
     additional scaled derivative.
  */
  rhup = 0.0;
  if (IGS.l == IGS.lmax) goto L540;

  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i)
    savf[i] = acor[i] - yh[i + IGS.lmax * yh_dim1];

  dup = vnorm_(&(IGS.n), &savf[1], &ewt[1]) / IGS.tesco[IGS.nq * 3 - 1];

  exup = 1.0 / (double) (IGS.l + 1);
  rhup = 1.0 / (pow(dup, exup) * 1.4 + 1.4e-6);

L540:
  exsm = 1.0 / (double) IGS.l;
  rhsm = 1.0 / (pow(dsm, exsm) * 1.2 + 1.2e-6);
  rhdn = 0.0;

  if (IGS.nq == 1) goto L560;

  ddn = vnorm_(&(IGS.n), &yh[IGS.l * yh_dim1 + 1],
               &ewt[1]) / IGS.tesco[IGS.nq * 3 - 3];

  exdn = 1.0 / (double) IGS.nq;
  rhdn = 1.0 / (pow(ddn, exdn) * 1.3 + 1.3e-6);

L560:
  if (rhsm >= rhup) goto L570;

  if (rhup > rhdn) goto L590;

  goto L580;

L570:
  if (rhsm < rhdn) goto L580;

  newq = IGS.nq;
  rh = rhsm;
  goto L620;

L580:
  newq = IGS.nq - 1;
  rh = rhdn;
  if (IGS.kflag < 0 && rh > 1.0) rh = 1.0;

  goto L620;

L590:
  newq = IGS.l;
  rh = rhup;
  if (rh < 1.1) goto L610;

  r = IGS.el[IGS.l - 1] / (double) IGS.l;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i)
    yh[i + (newq + 1) * yh_dim1] = acor[i] * r;

  goto L630;

L610:
  IGS.ialth = 3;
  goto L700;

L620:
  if (IGS.kflag == 0 && rh < 1.1) goto L610;

  if (IGS.kflag <= -2) rh = mymin(rh,0.2);

  /* --------------------------------------------------------------------------
     if there is a change of order, reset nq, l, and the coefficients.
     in any case h is reset according to rh and the yh array is rescaled.
     then exit from 690 if the step was ok, or redo the step otherwise.
  */
  if (newq == IGS.nq) goto L170;

L630:
  IGS.nq = newq;
  IGS.l = IGS.nq + 1;
  iret = 2;
  goto L150;

L640:
  /* --------------------------------------------------------------------------
     control reaches this section if 3 or more failures have occured.
     if 10 failures have occurred, exit with kflag = -1.
     it is assumed that the derivatives that have accumulated in the
     yh array have errors of the wrong order.  hence the first
     derivative is recomputed, and the order is set to 1.  then
     h is reduced by a factor of 10, and the step is retried,
     until it succeeds or h reaches hmin.
  */
  if (IGS.kflag == -10) goto L660;

  /* Computing MAX */
  d__1 = IGS.hmin / fabs(IGS.h);
  rh = mymax(d__1, 0.1);
  IGS.h *= rh;
  for (i = 1; i <= IGS.n; ++i) 
    y[i] = yh[i + yh_dim1];

  CalcDeriv (&y[1], &savf[1], &(IGS.tn));

  ++IGS.nfe;
  for (i = 1; i <= IGS.n; ++i)
    yh[i + (yh_dim1 << 1)] = IGS.h * savf[i];

  IGS.ipup = IGS.miter;
  IGS.ialth = 5;
  if (IGS.nq == 1) goto L200;

  IGS.nq = 1;
  IGS.l = 2;
  iret = 3;
  goto L150;

  /* --------------------------------------------------------------------------
     all returns are made through this section.  h is saved in hold
     to allow the caller to change h on the next step.
  */
L660:
  IGS.kflag = -1;
  goto L720;

L670:
  IGS.kflag = -2;
  goto L720;

L680:
  IGS.kflag = -3;
  goto L720;

L690:
  IGS.rmax = 10.0;

L700:
  r = 1.0 / IGS.tesco[IGS.nqu * 3 - 2];
  for (i = 1; i <= IGS.n; ++i)
    acor[i] *= r;

L720:
  IGS.hold = IGS.h;
  IGS.jstart = 1;

  return 0;

} /* end of stode_ */


/* ----------------------------------------------------------------------------
   vnorm

   this function routine computes the weighted root-mean-square norm
   of the vector of length n contained in the array v, with weights
   contained in the array w of length n..

   vnorm = sqrt( (1/n) * sum( v(i)*w(i) )**2 )
*/

double vnorm_(long *n, double *v, double *w)
{
  /* System generated locals */
  register double d__1;

  /* Local variables */
  register long i;
  register double sum;

  /* Function Body */
  sum = 0.0;
  for (i = 0; i < *n; ++i) {
    /* Computing 2nd power */
    d__1 = v[i] * w[i];
    sum += d__1 * d__1;
  }

  return sqrt(sum / (double) (*n));

} /* vnorm_ */


/* -----------------------------------------------------------------------------
   cfode

   is called by the integrator routine to set coefficients
   needed there.  the coefficients for the current method, as
   given by the value of meth, are set for all orders and saved.
   the maximum order assumed here is 12 if meth = 1 and 5 if meth = 2.
   (a smaller value of the maximum order is also allowed.)
   cfode is called once at the beginning of the problem,
   and is not called again unless and until meth is changed.

   the elco array contains the basic method coefficients.
   the coefficients el(i), 1 .le. i .le. nq+1, for the method of
   order nq are stored in elco(i,nq).  they are given by a genetrating
   polynomial, i.e.,
       l(x) = el(1) + el(2)*x + ... + el(nq+1)*x**nq.
   for the implicit adams methods, l(x) is given by
       dl/dx = (x+1)*(x+2)*...*(x+nq-1)/factorial(nq-1), l(-1) = 0.
   for the bdf methods, l(x) is given by
       l(x) = (x+1)*(x+2)* ... *(x+nq)/k,
   where k = factorial(nq)*(1 + 1/2 + ... + 1/nq).

   the tesco array contains test constants used for the
   local error test and the selection of step size and/or order.
   at order nq, tesco(k,nq) is used for the selection of step
   size at order nq - 1 if k = 1, at order nq if k = 2, and at order
   nq + 1 if k = 3.
*/

int cfode_(long *meth, double *elco, double * tesco)
{
  /* Local variables */
  double ragq, pint, xpin, fnqm1;
  long i;
  double agamq, rqfac, tsign, rq1fac;
  long ib;
  double pc[12];
  long nq;
  double fnq;
  long nqm1, nqp1;

  /* Parameter adjustments */
  tesco -= 4;
  elco -= 14;

  /* Function Body */
  switch (*meth) {
    case 1:  goto L100;
    case 2:  goto L200;
  }

L100:
  elco[14] = 1.;
  elco[15] = 1.;
  tesco[4] = 0.;
  tesco[5] = 2.;
  tesco[7] = 1.;
  tesco[39] = 0.;
  pc[0] = 1.;
  rqfac = 1.;
  for (nq = 2; nq <= 12; ++nq) {

    /* the pc array will contain the coefficients of the polynomial
       p(x) = (x+1)*(x+2)*...*(x+nq-1).
       initially, p(x) = 1. */

    rq1fac = rqfac;
    rqfac /= (double) nq;
    nqm1 = nq - 1;
    fnqm1 = (double) nqm1;
    nqp1 = nq + 1;
    /* form coefficients of p(x)*(x+nq-1) */
    pc[nq - 1] = 0.;

    for (ib = 1; ib <= nqm1; ++ib) {
      i = nqp1 - ib;
      pc[i - 1] = pc[i - 2] + fnqm1 * pc[i - 1];
    }
    pc[0] = fnqm1 * pc[0];

    /* compute integral, -1 to 0, of p(x) and x*p(x) */
    pint = pc[0];
    xpin = pc[0] / 2.;
    tsign = 1.;

    for (i = 2; i <= nq; ++i) {
      tsign = -tsign;
      pint += tsign * pc[i - 1] / (double) i;

      xpin += tsign * pc[i - 1] / (double) (i + 1);
    }

    /* store coefficients in elco and tesco */
    elco[nq * 13 + 1] = pint * rq1fac;
    elco[nq * 13 + 2] = 1.;

    for (i = 2; i <= nq; ++i)
      elco[i + 1 + nq * 13] = rq1fac * pc[i - 1] / (double) i;

    agamq = rqfac * xpin;
    ragq = 1.0 / agamq;
    tesco[nq * 3 + 2] = ragq;
    if (nq < 12)
      tesco[nqp1 * 3 + 1] = ragq * rqfac / (double) nqp1;

    tesco[nqm1 * 3 + 3] = ragq;
  }
  return 0;

L200:
  pc[0] = 1.;
  rq1fac = 1.;
  for (nq = 1; nq <= 5; ++nq) {

    /* the pc array will contain the coefficients of the polynomial
       p(x) = (x+1)*(x+2)*...*(x+nq).
       initially, p(x) = 1.
     */
    fnq = (double) nq;
    nqp1 = nq + 1;

    /* form coefficients of p(x)*(x+nq) */
    pc[nqp1 - 1] = 0.;

    for (ib = 1; ib <= nq; ++ib) {
      i = nq + 2 - ib;
      pc[i - 1] = pc[i - 2] + fnq * pc[i - 1];
    }
    pc[0] = fnq * pc[0];

    /* store coefficients in elco and tesco */
    for (i = 1; i <= nqp1; ++i)
      elco[i + nq * 13] = pc[i - 1] / pc[1];

    elco[nq * 13 + 2] = 1.;
    tesco[nq * 3 + 1] = rq1fac;
    tesco[nq * 3 + 2] = (double) nqp1 / elco[nq * 13 + 1];
    tesco[nq * 3 + 3] = (double) (nq + 2) / elco[nq * 13 + 1];
    rq1fac /= fnq;
  }

  return 0;

} /* cfode_ */


/* -----------------------------------------------------------------------------
   intdy_

   computes interpolated values of the k-th derivative of the
   dependent variable vector y, and stores it in dky.  this routine
   is called within the package with k = 0 and t = tout, but may
   also be called by the user for any k up to the current order.
   (see detailed instructions in the usage documentation.)

   the computed values in dky are gotten by interpolation using the
   nordsieck history array yh.  this array corresponds uniquely to a
   vector-valued polynomial of degree nqcur or less, and dky is set
   to the k-th derivative of this polynomial at t.
   the formula for dky is..
                q
    dky(i)  =  sum  c(j,k) * (t - tn)**(j-k) * h**(-j) * yh(i,j+1)
               j=k
   where  c(j,k) = j*(j-1)*...*(j-k+1), q = nqcur, tn = tcur, h = hcur.

   the quantities  nq = nqcur, l = nq+1, n = neq, tn, and h are
   communicated by common.  the above sum is done in reverse order.
   iflag is returned negative if either k or t is out of bounds.
*/

int intdy_(double *t, long k, double *yh,
           long *nyh, double *dky, long *iflag)
{
  /* System generated locals */
  long yh_dim1, yh_offset, i__1;

  /* Local variables */
  double c;
  long i, j;
  double r, s;
  long ic, jb, jj;
  double tp;
  long jb2, jj1, jp1;

  /* Parameter adjustments */
  --dky;
  yh_dim1 = *nyh;
  yh_offset = yh_dim1 + 1;
  yh -= yh_offset;

  *iflag = 0;
  if (k < 0 || k > IGS.nq) goto L80;

  tp = IGS.tn - IGS.hu - IGS.uround * 100. * (IGS.tn + IGS.hu);
  if ((*t - tp) * (*t - IGS.tn) > 0.) goto L90;

  s = (*t - IGS.tn) / IGS.h;
  ic = 1;
  if (k == 0)
    goto L15;

  jj1 = IGS.l - k;
  i__1 = IGS.nq;
  for (jj = jj1; jj <= i__1; ++jj)
    ic *= jj;

L15:
  c = (double) ic;
  i__1 = IGS.n;
  for (i = 1; i <= i__1; ++i)
    dky[i] = c * yh[i + IGS.l * yh_dim1];
  if (k == IGS.nq) goto L55;

  jb2 = IGS.nq - k;
  for (jb = 1; jb <= jb2; ++jb) {
    j = IGS.nq - jb;
    jp1 = j + 1;
    ic = 1;
    if (k != 0) {
      jj1 = jp1 - k;
      for (jj = jj1; jj <= j; ++jj) ic *= jj;
    }

    c = (double) ic;
    for (i = 1; i <= IGS.n; ++i)
      dky[i] = c * yh[i + jp1 * yh_dim1] + s * dky[i];
  }
  if (k == 0) return 0;

L55:
  r = pow(IGS.h,(double) -k);
  for (i = 1; i <= IGS.n; ++i) dky[i] = r * dky[i];
  return 0;

L80:
  printf ("intdy: k = %ld is illegal.\n", k);
  *iflag = -1;
  return 0;

L90:
  printf ("intdy: t = %21.13f is illegal, it is not in the interval", *t);
  printf ("       tcur - hu (= %21.13f) to tcur (= %21.13f).\n", tp, IGS.tn);
  *iflag = -2;
  return 0;

} /* end of routine intdy_ */


/* -----------------------------------------------------------------------------
   iprep_

   this routine serves as an interface between the driver and
   subroutine prep.  it is called only if miter is 1 or 2.
   tasks performed here are..
    * call prep,
    * reset the required wm segment length lenwk,
    * move yh back to its final location (following wm in rwork),
    * reset pointers for yh, savf, ewt, and acor, and
    * move ewt to its new position if istate = 1.
   ipflag is an output error indication flag.  ipflag = 0 if there was
   no trouble, and ipflag is the value of the prep error flag ipper
   if there was trouble in subroutine prep.
*/
int iprep_(long *neq, double *y, double *rwork,
           long *ia, long *ja, long *ipflag)
{
    /* System generated locals */
    long i__1;

    /* Local variables */
    long imax, lyhd, lyhn;
    long i, lewtn;

    /* Parameter adjustments */
    --ja;
    --ia;
    --rwork;
    --y;
    --neq;

    /* Function Body */
    *ipflag = 0;
    /* call prep to do matrix preprocessing operations */
    prep_(&neq[1], &y[1], &rwork[IGS.lyh], &rwork[IGS.lsavf], &rwork[IGS.lewt],
          &rwork[IGS.lacor], &ia[1], &ja[1], &rwork[IGS.lwm],
          (long *)&rwork[IGS.lwm], ipflag);
    IGS.lenwk = mymax(IGS.lreq,IGS.lwmin);
    if (*ipflag < 0)
      return 0;

    /* if prep was successful, move yh to end of required space for wm */
    lyhn = IGS.lwm + IGS.lenwk;
    if (lyhn > IGS.lyh)
      return 0;
    
    lyhd = IGS.lyh - lyhn;
    if (lyhd == 0) 
      goto L20;
    
    imax = lyhn - 1 + IGS.lenyhm;
    i__1 = imax;
    for (i = lyhn; i <= i__1; ++i)
      rwork[i] = rwork[i + lyhd];

    IGS.lyh = lyhn;
    /* reset pointers for savf, ewt, and acor */
L20:
    IGS.lsavf = IGS.lyh + IGS.lenyh;
    lewtn = IGS.lsavf + IGS.n;
    IGS.lacor = lewtn + IGS.n;
    if (IGS.istatc == 3)
      goto L40;

    /* if istate = 1, move ewt (left) to its new position */
    if (lewtn > IGS.lewt)
      return 0;

    i__1 = IGS.n;
    for (i = 1; i <= i__1; ++i)
      rwork[i + lewtn - 1] = rwork[i + IGS.lewt - 1];

L40:
    IGS.lewt = lewtn;
    return 0;

} /* iprep_ */


/* --------------------------------------------------------------------------
   _prep

   this routine performs preprocessing related to the sparse linear 
   systems that must be solved if miter = 1 or 2. 
   the operations that are performed here are.. 
    * compute sparseness structure of jacobian according to moss, 
    * compute grouping of column indices (miter = 2), 
    * compute a new ordering of rows and columns of the matrix, 
    * reorder ja corresponding to the new ordering, 
    * perform a symbolic lu factorization of the matrix, and 
    * set pointers for segments of the iwk/wk array. 
   in addition to variables described previously, prep uses the 
   following for communication.. 
   yh     = the history array.  only the first column, containing the 
            current y vector, is used.  used only if moss .ne. 0. 
   savf   = a work array of length neq, used only if moss .ne. 0. 
   ewt    = array of length neq containing (inverted) error weights. 
            used only if moss = 2 or if istate = moss = 1. 
   ftem   = a work array of length neq, identical to acor in the driver,

            used only if moss = 2. 
   wk     = a real work array of length lenwk, identical to wm in 
            the driver. 
   iwk    = long work array, assumed to occupy the same space as wk. 

   lenwk  = the length of the work arrays wk and iwk. 
   istatc = a copy of the driver input argument istate (= 1 on the 
            first call, = 3 on a continuation call). 
   iys    = flag value from odrv or cdrv. 
   ipper  = output error flag with the following values and meanings.. 
            0  no error. 
           -1  insufficient storage for internal structure pointers. 
           -2  insufficient storage for jgroup. 
           -3  insufficient storage for odrv. 
           -4  other error flag from odrv (should never occur). 
           -5  insufficient storage for cdrv. 
           -6  other error flag from cdrv. 
*/
int prep_(long *neq, double *y, double *yh,
          double *savf, double *ewt, double *ftem, long *ia,
          long *ja, double *wk, long *iwk, long *ipper)
{
    /* System generated locals */
    long i__1, i__2;

    /* Builtin functions */
    double d_sign(double *, double *);

    /* Local variables */
    long ldif, ipil, knew, ipiu, kmax, kmin, liwk, maxg;
    double erwt;
    long iptt1, iptt2, i, j, k;
    long nzsut;
    double dq, yj;
    long lenigp, jfound;
    long np1;
    double fac;
    long ibr, ier;
    double dyj;

    /* Parameter adjustments */
    --iwk;
    --wk;
    --ja;
    --ia;
    --ftem;
    --ewt;
    --savf;
    --yh;
    --y;
    --neq;

    /* Function Body */
    IGS.ibian = IGS.lrat << 1;
    IGS.ipian = IGS.ibian + 1;
    np1 = IGS.n + 1;
    IGS.ipjan = IGS.ipian + np1;
    IGS.ibjan = IGS.ipjan - 1;
    liwk = IGS.lenwk * IGS.lrat;
    if (IGS.ipjan + IGS.n - 1 > liwk) goto L210;

    if (IGS.moss == 0) goto L30;

    if (IGS.istatc == 3) goto L20;

    /* istate = 1 and moss .ne. 0.  perturb y for structure determination */
    i__1 = IGS.n;
    for (i = 1; i <= i__1; ++i) {
    erwt = 1. / ewt[i];
    fac = 1. / ((double) i + 1.) + 1.;
    y[i] += fac * d_sign(&erwt, &y[i]);

    }
    switch (IGS.moss) {
      case 1:  goto L70;
      case 2:  goto L100;
    }

L20:
    /* istate = 3 and moss .ne. 0.  load y from yh(*,1) */
    for (i = 1; i <= IGS.n; ++i)
      y[i] = yh[i];
 
    switch (IGS.moss) {
      case 1:  goto L70;
      case 2:  goto L100;
    }

    /* moss = 0. Process user-s ia,ja.  add diagonal entries if necessary */
L30:
    knew = IGS.ipjan;
    kmin = ia[1];
    iwk[IGS.ipian] = 1;
    i__1 = IGS.n;
    for (j = 1; j <= i__1; ++j) {
      jfound = 0;
      kmax = ia[j + 1] - 1;
      if (kmin > kmax)
        goto L45;
   
      i__2 = kmax;
      for (k = kmin; k <= i__2; ++k) {
        i = ja[k];
        if (i == j)
          jfound = 1;

        if (knew > liwk) goto L210;
 
        iwk[knew] = i;
        ++knew;

      }
      if (jfound == 1) goto L50;

L45:
      if (knew > liwk) goto L210;

      iwk[knew] = j;
      ++knew;
L50:
      iwk[IGS.ipian + j] = knew + 1 - IGS.ipjan;
      kmin = kmax + 1;

    }

    goto L140;

    /* moss = 1. Compute structure from user-supplied jacobian routine jac */
L70:

    /* a dummy call to f allows user to create temporaries for use in jac */
    CalcDeriv (&y[1], &savf[1], &(IGS.tn));

    k = IGS.ipjan;
    iwk[IGS.ipian] = 1;

    i__1 = IGS.n;
    for (j = 1; j <= i__1; ++j) {
      if (k > liwk) goto L210;

      iwk[k] = j;
      ++k;
      for (i = 1; i <= IGS.n; ++i)
        savf[i] = 0.;
 
      CalcJacob (&(IGS.tn), &y[1], j, &savf[1]);

      i__2 = IGS.n;
      for (i = 1; i <= i__2; ++i) {
        if (fabs (savf[i]) <= IGS.seth)
          goto L80;

        if (i == j)
          goto L80;

        if (k > liwk) 
          goto L210;

        iwk[k] = i;
        ++k;
L80:
        ;
      } /* end for i */
      iwk[IGS.ipian + j] = k + 1 - IGS.ipjan;

    } /* end for j */
    goto L140;

    /* moss = 2.  compute structure from results of n + 1 calls to f. */
L100:
    k = IGS.ipjan;
    iwk[IGS.ipian] = 1;

    CalcDeriv (&y[1], &savf[1], &(IGS.tn));

    i__1 = IGS.n;
    for (j = 1; j <= i__1; ++j) {
      if (k > liwk)
        goto L210;
     
      iwk[k] = j;
      ++k;
      yj = y[j];
      erwt = 1. / ewt[j];
      dyj = d_sign(&erwt, &yj);
      y[j] = yj + dyj;

      CalcDeriv (&y[1], &ftem[1], &(IGS.tn));

      y[j] = yj;

      i__2 = IGS.n;
      for (i = 1; i <= i__2; ++i) {
        dq = (ftem[i] - savf[i]) / dyj;
        if (fabs(dq) <= IGS.seth)
          goto L110;

        if (i == j)
          goto L110;

        if (k > liwk)
          goto L210;

        iwk[k] = i;
        ++k;
L110:
        ;
      } /* end for i */

      iwk[IGS.ipian + j] = k + 1 - IGS.ipjan;
    } /* end for j */

L140:
    if (IGS.moss == 0 || IGS.istatc != 1)
      goto L150;

    /* if istate = 1 and moss .ne. 0, restore y from yh */
    for (i = 1; i <= IGS.n; ++i)
      y[i] = yh[i];

L150:
    IGS.nnz = iwk[IGS.ipian + IGS.n] - 1;
    lenigp = 0;
    IGS.ipigp = IGS.ipjan + IGS.nnz;
    if (IGS.miter != 2)
      goto L160;

    /* compute grouping of column indices (miter = 2) */
    maxg = np1;
    IGS.ipjgp = IGS.ipjan + IGS.nnz;
    IGS.ibjgp = IGS.ipjgp - 1;
    IGS.ipigp = IGS.ipjgp + IGS.n;
    iptt1 = IGS.ipigp + np1;
    iptt2 = iptt1 + IGS.n;
    IGS.lreq = iptt2 + IGS.n - 1;
    if (IGS.lreq > liwk)
      goto L220;

    jgroup_(&(IGS.n), &iwk[IGS.ipian], &iwk[IGS.ipjan], &maxg, &(IGS.ngp), 
            &iwk[IGS.ipigp], &iwk[IGS.ipjgp], &iwk[iptt1], &iwk[iptt2], &ier);

    if (ier != 0)
      goto L220;

    lenigp = IGS.ngp + 1;

    /* compute new ordering of rows/columns of jacobian */
L160:
    IGS.ipr = IGS.ipigp + lenigp;
    IGS.ipc = IGS.ipr;
    IGS.ipic = IGS.ipc + IGS.n;
    IGS.ipisp = IGS.ipic + IGS.n;
    IGS.iprsp = (IGS.ipisp - 2) / IGS.lrat + 2;
    IGS.iesp = IGS.lenwk + 1 - IGS.iprsp;
    if (IGS.iesp < 0)
      goto L230;

    ibr = IGS.ipr - 1;
    for (i = 1; i <= IGS.n; ++i)
      iwk[ibr + i] = i;

    IGS.nsp = liwk + 1 - IGS.ipisp;

    odrv_(&(IGS.n), &iwk[IGS.ipian], &iwk[IGS.ipjan], &wk[1], &iwk[IGS.ipr],
          &iwk[IGS.ipic], &(IGS.nsp), &iwk[IGS.ipisp], 1, &(IGS.iys));

    if (IGS.iys == IGS.n * 11 + 1)
      goto L240;

    if (IGS.iys != 0) 
      goto L230;

    /* reorder jan and do symbolic lu factorization of matrix */
    IGS.ipa = IGS.lenwk + 1 - IGS.nnz;
    IGS.nsp = IGS.ipa - IGS.iprsp;

    /* Computing MAX */
    i__1 = IGS.n * 12 / IGS.lrat, i__2 = IGS.n * 6 / IGS.lrat +
                                         (IGS.n << 1) + IGS.nnz;
    IGS.lreq = mymax(i__1,i__2) + 3;
    IGS.lreq = IGS.lreq + IGS.iprsp - 1 + IGS.nnz;
    if (IGS.lreq > IGS.lenwk)
      goto L250;

    IGS.iba = IGS.ipa - 1;
    for (i = 1; i <= IGS.nnz; ++i)
      wk[IGS.iba + i] = 0.0;

    IGS.ipisp = IGS.lrat * (IGS.iprsp - 1) + 1;
    cdrv_(&(IGS.n), &iwk[IGS.ipr], &iwk[IGS.ipc], &iwk[IGS.ipic], &iwk[IGS.ipian], 
          &iwk[IGS.ipjan], &wk[IGS.ipa], &wk[IGS.ipa], &wk[IGS.ipa], &(IGS.nsp), 
          &iwk[IGS.ipisp], &wk[IGS.iprsp], &(IGS.iesp), 5, &(IGS.iys));
    IGS.lreq = IGS.lenwk - IGS.iesp;
    if (IGS.iys == IGS.n * 10 + 1)
      goto L250;

    if (IGS.iys != 0)
      goto L260;

    ipil = IGS.ipisp;
    ipiu = ipil + (IGS.n << 1) + 1;
    IGS.nzu = iwk[ipil + IGS.n] - iwk[ipil];
    IGS.nzl = iwk[ipiu + IGS.n] - iwk[ipiu];
    if (IGS.lrat > 1) 
      goto L190;

    adjlr_(&(IGS.n), &iwk[IGS.ipisp], &ldif);
    IGS.lreq += ldif;

L190:
    if (IGS.lrat == 2 && IGS.nnz == IGS.n)
      ++IGS.lreq;

    IGS.nsp = IGS.nsp + IGS.lreq - IGS.lenwk;
    IGS.ipa = IGS.lreq + 1 - IGS.nnz;
    IGS.iba = IGS.ipa - 1;
    *ipper = 0;
    return 0;

L210:
    *ipper = -1;
    IGS.lreq = ((IGS.n << 1) + 1) / IGS.lrat + 2;

    /* Computing MAX */
    i__1 = IGS.lenwk + 1;
    IGS.lreq = mymax(i__1,IGS.lreq);
    return 0;

L220:
    *ipper = -2;
    IGS.lreq = (IGS.lreq - 1) / IGS.lrat + 1;
    return 0;

L230:
    *ipper = -3;
    cntnzu_(&(IGS.n), &iwk[IGS.ipian], &iwk[IGS.ipjan], &nzsut);
    IGS.lreq = IGS.lenwk - IGS.iesp + (IGS.n * 3 + (nzsut << 2) - 1) / IGS.lrat + 1;
    return 0;

L240:
    *ipper = -4;
    return 0;

L250:
    *ipper = -5;
    return 0;

L260:
    *ipper = -6;
    IGS.lreq = IGS.lenwk;
    return 0;

} /* prep_ */


/* --------------------------------------------------------------------------
   cntnzu_

   This routine counts the number of nonzero elements in the strict 
   upper triangle of the matrix m + m(transpose), where the sparsity 
   structure of m is given by pointer arrays ia and ja. 
   this is needed to compute the storage requirements for the 
   sparse matrix reordering operation in odrv. 
 */

int cntnzu_(long *n, long *ia, long *ja, long *nzsut) {

  /* System generated locals */
  long i__1, i__2, i__3;

  /* Local variables */
  long jmin, kmin, jmax, kmax, j, k, ii, jj, num;

  /* Parameter adjustments */
  --ja;
  --ia;

  /* Function Body */
  num = 0;
  i__1 = *n;
  for (ii = 1; ii <= i__1; ++ii) {
    jmin = ia[ii];
    jmax = ia[ii + 1] - 1;
    if (jmin > jmax) goto L50;

    i__2 = jmax;
    for (j = jmin; j <= i__2; ++j) {

      if ((i__3 = ja[j] - ii) < 0) {
        goto L10;
      }
      else {
        if (i__3 == 0) {
          goto L40;
        }
        else {
         goto L30;
        }
      }
L10:
      jj = ja[j];
      kmin = ia[jj];
      kmax = ia[jj + 1] - 1;
      if (kmin > kmax) goto L30;

      i__3 = kmax;
      for (k = kmin; k <= i__3; ++k)
        if (ja[k] == ii) goto L40;

L30:
      ++num;
L40:
      ;
    } /* for j */
L50:
    ;
  } /* for ii */

  *nzsut = num;
  return 0;

} /* cntnzu_ */


/* -----------------------------------------------------------------------------
   adjlr_

   this routine computes an adjustment, ldif, to the required
   long storage space in iwk (sparse matrix work space).
   it is called only if the word length ratio is lrat = 1.
   this is to account for the possibility that the symbolic lu phase
   may require more storage than the numerical lu and solution phases.
*/
int adjlr_(long *n, long *isp, long *ldif)
{
  /* Local variables */
  long lnfc, lsfc, nzlu, jlmax, jumax, ip;

  /* Parameter adjustments */
  --isp;

  /* Function Body */
  ip = (*n << 1) + 1;
  jlmax = isp[ip];
  jumax = isp[ip + ip];
  nzlu = isp[*n + 1] - isp[1] + isp[ip + *n + 1] - isp[ip + 1];
  lsfc = *n * 12 + 3 + (mymax(jlmax,jumax) << 1);
  lnfc = *n * 9 + 2 + jlmax + jumax + nzlu;

  /* Computing MAX */
  *ldif = lsfc - lnfc;
  if (*ldif < 0) *ldif = 0;

  return 0;

} /* adjlr_ */


/* -----------------------------------------------------------------------------
   cdrv

   driver for subroutines for solving sparse nonsymmetric systems

   for several systems whose coefficient matrices have the same
    nonzero structure, nsfc need be done only once (for the first
   system).  then nnfc is done once for each additional system.  for
   several systems with the same coefficient matrix, nsfc and nnfc
   need be done only once (for the first system).  then nnsc or nntc
   is done once for each additional right-hand side.

   nv    - path  - path specification.  values and their meanings are --
         -           1  perform nroc, nsfc, and nnfc.
         -           2  perform nnfc only  (nsfc is assumed to have been
         -               done in a manner compatible with the storage
         -               allocation used in the driver).
         -           3  perform nnsc only  (nsfc and nnfc are assumed to
         -               have been done in a manner compatible with the
         -               storage allocation used in the driver).
         -           4  perform nntc only  (nsfc and nnfc are assumed to
         -               have been done in a manner compatible with the
         -               storage allocation used in the driver).
         -           5  perform nroc and nsfc.

   various errors are detected by the driver and the individual
   subroutines.

   nr    - flag  - error flag.  values and their meanings are --
         -             0     no errors detected
         -             n+k   null row in a  --  row = k
         -            2n+k   duplicate entry in a  --  row = k
         -            3n+k   insufficient storage in nsfc  --  row = k
         -            4n+1   insufficient storage in nnfc
         -            5n+k   null pivot  --  row = k
         -            6n+k   insufficient storage in nsfc  --  row = k
         -            7n+1   insufficient storage in nnfc
         -            8n+k   zero pivot  --  row = k
         -           10n+1   insufficient storage in cdrv
         -           11n+1   illegal path specification

    working storage is needed for the factored form of the matrix
    m plus various temporary vectors.  the arrays isp and rsp should be
    equivalenced.  long storage is allocated from the beginning of
    isp and real storage from the end of rsp.

   nv    - nsp   - declared dimension of rsp.  nsp generally must
         -           be larger than  8n+2 + 2k  (where  k = (number of
         -           nonzero entries in m)).
   nvira - isp   - long working storage divided up into various arrays
         -           needed by the subroutines.  isp and rsp should be
         -           equivalenced.
         -           size = lratio*nsp.
   fvira - rsp   - real working storage divided up into various arrays
         -           needed by the subroutines.  isp and rsp should be
         -           equivalenced.
         -           size = nsp.
   nr    - esp   - if sufficient storage was available to perform the
         -           symbolic factorization (nsfc), then esp is set to
         -           the amount of excess storage provided (negative if
         -           insufficient storage was available to perform the
         -           numeric factorization (nnfc)).


  conversion to double precision

  set lratio equal to the ratio between the length of floating point
  and long array data.  e. g., lratio = 1 for (real, long),
  lratio = 2 for (double precision, long)
*/

int cdrv_(long *n, long *r, long *c, long *ic,
          long *ia, long *ja, double *a, double *b, double *z,
          long *nsp, long *isp, double *rsp, long *esp,
          long path, long *flag_)
{
  /* Local variables */
  long irac;
  long lmax, umax, d, i, j, l, q, u, jlmax, jumax, jutmp, ar, il, jl,
       iu, ju, ira, jra, ijl, max_, irl, iju, jrl, iru, tmp, jru, row;

  long lratio;
  lratio = sizeof(double)/sizeof(long);
  if (lratio < 1) lratio = 1;

  /* Parameter adjustments */
  --rsp;
  --isp;
  --z;
  --b;
  --a;
  --ja;
  --ia;
  --ic;
  --c;
  --r;

  /* Function Body */

  if (path < 1 || 5 < path) {
    /* error.. illegal path specification */
    *flag_ = *n * 11 + 1;
    return 0;
  }

  /*initialize and divide up temporary storage */
  il = 1;
  ijl = il + (*n + 1);
  iu = ijl + *n;
  iju = iu + (*n + 1);
  irl = iju + *n;
  jrl = irl + *n;
  jl = jrl + *n;

  /* reorder a if necessary, call nsfc if flag is set */
  if ((path - 1) * (path - 5) != 0) goto L5;

  max_ = lratio * *nsp + 1 - jl - (*n + 1) - *n * 5;
  jlmax = max_ / 2;
  q = jl + jlmax;
  ira = q + (*n + 1);
  jra = ira + *n;
  irac = jra + *n;
  iru = irac + *n;
  jru = iru + *n;
  jutmp = jru + *n;
  jumax = lratio * *nsp + 1 - jutmp;
  *esp = max_ / lratio;
  if (jlmax <= 0 || jumax <= 0) goto L110;


  for (i = 1; i <= *n; ++i)
    if (c[i] != i) goto L2;

  goto L3;
L2:
  ar = *nsp + 1 - *n;
  nroc_(n, &ic[1], &ia[1], &ja[1], &a[1], &isp[il], &rsp[ar], &isp[iu],
      flag_);
  if (*flag_ != 0) goto L100;

L3:
  nsfc_(n, &r[1], &ic[1], &ia[1], &ja[1], &jlmax, &isp[il], &isp[jl], &isp[
      ijl], &jumax, &isp[iu], &isp[jutmp], &isp[iju], &isp[q], &isp[ira]
    , &isp[jra], &isp[irac], &isp[irl], &isp[jrl], &isp[iru], &isp[
    jru], flag_);
  if (*flag_ != 0) {
  goto L100;
  }
  /* move ju next to jl */
  jlmax = isp[ijl + *n - 1];
  ju = jl + jlmax;
  jumax = isp[iju + *n - 1];
  if (jumax <= 0) {
  goto L5;
  }
  for (j = 1; j <= jumax; ++j) {

  isp[ju + j - 1] = isp[jutmp + j - 1];
  }

   /* call remaining subroutines */
L5:
  jlmax = isp[ijl + *n - 1];
  ju = jl + jlmax;
  jumax = isp[iju + *n - 1];
  l = (ju + jumax - 2 + lratio) / lratio + 1;
  lmax = isp[il + *n] - 1;
  d = l + lmax;
  u = d + *n;
  row = *nsp + 1 - *n;
  tmp = row - *n;
  umax = tmp - u;
  *esp = umax - (isp[iu + *n] - 1);

  if ((path - 1) * (path - 2) != 0) goto L6;

  if (umax < 0) goto L110;

  nnfc_(n, &r[1], &c[1], &ic[1], &ia[1], &ja[1], &a[1], &z[1], &b[1], &lmax,
        &isp[il], &isp[jl], &isp[ijl], &rsp[l], &rsp[d], &umax, &isp[iu],
        &isp[ju], &isp[iju], &rsp[u], &rsp[row], &rsp[tmp], &isp[irl],
        &isp[jrl], flag_);
  if (*flag_ != 0) goto L100;


L6:
  if (path - 3 != 0) goto L7;

  nnsc_(n, &r[1], &c[1], &isp[il], &isp[jl], &isp[ijl], &rsp[l], &rsp[d], &
    isp[iu], &isp[ju], &isp[iju], &rsp[u], &z[1], &b[1], &rsp[tmp]);

L7:
  if (path - 4 != 0) goto L8;

  nntc_(n, &r[1], &c[1], &isp[il], &isp[jl], &isp[ijl], &rsp[l], &rsp[d], &
        isp[iu], &isp[ju], &isp[iju], &rsp[u], &z[1], &b[0], &rsp[tmp]);
L8:
  return 0;

  /* error.. error detected in nroc, nsfc, nnfc, or nnsc */
L100:
   return 0;

  /* error.. insufficient storage */
L110:
  *flag_ = *n * 10 + 1;
  return 0;

} /* cdrv_ */


/* ----------------------------------------------------------------------------
   subroutine nntc

   numeric solution of the transpose of a sparse nonsymmetric system
   of linear equations given lu-factorization (compressed pointer
   storage)

   input variables..  n, r, c, il, jl, ijl, l, d, iu, ju, iju, u, b

   output variables.. z

   parameters used internally..
   fia   - tmp   - temporary vector which gets result of solving ut y = b
         -         size = n.

   internal variables..
   jmin, jmax - indices of the first and last positions in a row of
                u or l  to be used.
*/
int nntc_(long *n, long *r, long *c, long *il,
          long *jl, long *ijl, double *l, double *d, long *iu,
          long *ju, long *iju, double *u, double *z,
          double *b, double *tmp)
{
  /* Local variables */
  long jmin, jmax;
  double tmpk;
  long i, j, k, ml, mu;
  double sum;

  /* set tmp to reordered b */
  for (k = 0; k < *n; ++k) tmp[k] = b[c[k]];

  /* solve Ut y = b  by forward substitution */
  for (k = 0; k < *n; ++k) {
    jmin = iu[k];
    jmax = iu[k + 1] - 1;
    tmpk = -tmp[k];
    if (jmin <= jmax) {
      mu = iju[k] - jmin;
      for (j = jmin - 1; j < jmax; ++j) tmp[ju[mu + j] - 1] += tmpk * u[j];
    }
  } /* end for k */

  /* solve lt x = y by back substitution */
  k = *n - 1;
  for (i = 0; i < *n; ++i) {
    sum = -tmp[k];
    jmin = il[k];
    jmax = il[k + 1] - 1;
    if (jmin <= jmax) {
      ml = ijl[k] - jmin;
      for (j = jmin - 1; j < jmax; ++j) sum += l[j] * tmp[jl[ml + j] - 1];
    }

    tmp[k] = -sum * d[k];
    z[r[k]-1] = tmp[k];
    --k;

  } /* end for k */

  return 0;

} /* nntc_ */


/* ----------------------------------------------------------------------------
   nnsc

   numerical solution of sparse nonsymmetric system of linear
   equations given ldu-factorization (compressed pointer storage)

   input variables..  n, r, c, il, jl, ijl, l, d, iu, ju, iju, u, b
   output variables.. z

   parameters used internally..
   fia   - tmp   - temporary vector which gets result of solving  ly = b.
         -           size = n.

   internal variables..
   jmin, jmax - indices of the first and last positions in a row of
                u or l  to be used.
*/

int nnsc_(long *n, long *r, long *c, long *il,
          long *jl, long *ijl, double *l, double *d, long *iu,
          long *ju, long *iju, double *u, double *z,
          double *b, double *tmp)
{
  /* Local variables */
  long jmin, jmax;
  double tmpk;
  long i, j, k, ml, mu;
  double sum;

  /* Parameter adjustments */
  --tmp;
  --b;
  --z;
  --u;
  --iju;
  --ju;
  --iu;
  --d;
  --l;
  --ijl;
  --jl;
  --il;
  --c;
  --r;

  /* Function Body */

  /* set tmp to reordered b */
  for (k = 1; k <= *n; ++k) tmp[k] = b[r[k]];

  /* solve  ly = b  by forward substitution */
  for (k = 1; k <= *n; ++k) {
    jmin = il[k];
    jmax = il[k + 1] - 1;
    tmpk = -d[k] * tmp[k];
    tmp[k] = -tmpk;
    if (jmin <= jmax) {
      ml = ijl[k] - jmin;
      for (j = jmin; j <= jmax; ++j) tmp[jl[ml + j]] += tmpk * l[j];
    }
  } /* end for k */

  /* solve  ux = y  by back substitution */
  k = *n;
  for (i = 1; i <= *n; ++i) {
    sum = -tmp[k];
    jmin = iu[k];
    jmax = iu[k + 1] - 1;
    if (jmin <= jmax) {
      mu = iju[k] - jmin;
      for (j = jmin; j <= jmax; ++j) sum += u[j] * tmp[ju[mu + j]];
    }
    tmp[k] = -sum;
    z[c[k]] = -sum;
    --k;

  } /* end for i */

  return 0;

} /* nnsc_ */


/* -----------------------------------------------------------------------------
   nnfc

   numerical ldu-factorization of sparse nonsymmetric matrix and
   solution of system of linear equations (compressed pointer
   storage)


   input variables..  n, r, c, ic, ia, ja, a, b,
                      il, jl, ijl, lmax, iu, ju, iju, umax

   output variables.. z, l, d, u, flag

   parameters used internally..
   nia   - irl,  - vectors used to find the rows of  l.  at the kth step

   nia   - jrl       of the factorization,  jrl(k)  points to the head
         -           of a linked list in  jrl  of column indices j
         -           such j .lt. k and  l(k,j)  is nonzero.  zero
         -           indicates the end of the list.  irl(j)  (j.lt.k)
         -           points to the smallest i such that i .ge. k and
         -           l(i,j)  is nonzero.
         -           size of each = n.
   fia   - row   - holds intermediate values in calculation of  u and l.
         -           size = n.
   fia   - tmp   - holds new right-hand side  b*  for solution of the
         -           equation ux = b*.
         -           size = n.

   internal variables..
   jmin, jmax - indices of the first and last positions in a row to
                be examined.
   sum - used in calculating  tmp.
*/

int nnfc_(long *n, long *r, long *c, long *ic,
          long *ia, long *ja, double *a, double *z, double *b,
          long *lmax, long *il, long *jl, long *ijl, double *l,
          double *d, long *umax, long *iu, long *ju, long *iju,
          double *u, double *row, double *tmp, long *irl,
          long *jrl, long *flag_)
{
  /* Local variables */
  long ijlb, jmin, jmax, i, j, k, i1, i2;
  double dk;
  long rk, mu;
  double lki, sum;

  /* Parameter adjustments */
  --jrl;
  --irl;
  --tmp;
  --row;
  --u;
  --iju;
  --ju;
  --iu;
  --d;
  --l;
  --ijl;
  --jl;
  --il;
  --b;
  --z;
  --a;
  --ja;
  --ia;
  --ic;
  --c;
  --r;

  /* Function Body */

  if (il[*n + 1] - 1 > *lmax) {
    /* error.. insufficient storage for l */
    *flag_ = (*n << 2) + 1;
    return 0;
  }

  if (iu[*n + 1] - 1 > *umax) {
    /* error.. insufficient storage for u */
    *flag_ = (*n) * 7 + 1;
    return 0;
  }

  for (k = 1; k <= *n; ++k) {
    irl[k] = il[k];
    jrl[k] = 0;
  }

  /* for each row */
  for (k = 1; k <= *n; ++k) {
    /* reverse jrl and zero row where kth row of l will fill in */
    row[k] = 0.;
    i1 = 0;
    if (jrl[k] == 0) goto L3;
    i = jrl[k];

L2:
    i2 = jrl[i];
    jrl[i] = i1;
    i1 = i;
    row[i] = 0.;
    i = i2;
    if (i != 0) goto L2;

L3:
    /* set row to zero where u will fill in */
    jmin = iju[k];
    jmax = jmin + iu[k + 1] - iu[k] - 1;
    if (jmin <= jmax)
      for (j = jmin; j <= jmax; ++j) row[ju[j]] = 0.;

    /*  place kth row of a in row */
    rk = r[k];
    jmin = ia[rk];
    jmax = ia[rk + 1] - 1;

    for (j = jmin; j <= jmax; ++j) row[ic[ja[j]]] = a[j];

    /* initialize sum, and link through jrl */
    sum = b[rk];
    i = i1;
    if (i == 0) goto L10;

L7:
    /* assign the kth row of l and adjust row, sum */
    lki = -row[i];
    /* if l is not required, then comment out the following line */
    l[irl[i]] = -lki;
    sum += lki * tmp[i];
    jmin = iu[i];
    jmax = iu[i + 1] - 1;
    if (jmin <= jmax) {
      mu = iju[i] - jmin;
      for (j = jmin; j <= jmax; ++j) row[ju[mu + j]] += lki * u[j];
    }

    i = jrl[i];
    if (i != 0) goto L7;

L10:
    /* assign kth row of u and diagonal d, set tmp(k) */

    if (row[k] == 0.) {
      /* error.. zero pivot */
      *flag_ = (*n << 3) + k;
      return 0;
    }

    dk = 1.0 / row[k];
    d[k] = dk;
    tmp[k] = sum * dk;
    if (k == *n) break;

    jmin = iu[k];
    jmax = iu[k + 1] - 1;
    if (jmin <= jmax) {
      mu = iju[k] - jmin;
      for (j = jmin; j <= jmax; ++j) u[j] = row[ju[mu + j]] * dk;
    }

    /* update irl and jrl, keeping jrl in decreasing order */
    i = i1;
    if (i == 0) goto L18;

L14:
    ++irl[i];
    i1 = jrl[i];
    if (irl[i] >= il[i + 1]) goto L17;

    ijlb = irl[i] - il[i] + ijl[i];
    j = jl[ijlb];

L15:
    if (i > jrl[j]) goto L16;
    j = jrl[j];
    goto L15;

L16:
    jrl[i] = jrl[j];
    jrl[j] = i;

L17:
    i = i1;
    if (i != 0) goto L14;

L18:
    if (irl[k] < il[k + 1]) {
      j = jl[ijl[k]];
      jrl[k] = jrl[j];
      jrl[j] = k;
    }

  } /* end of for k */

  /* solve ux = tmp by back substitution */
  k = *n;
  for (i = 1; i <= *n; ++i) {
    sum = tmp[k];
    jmin = iu[k];
    jmax = iu[k + 1] - 1;
    if (jmin <= jmax) {
      mu = iju[k] - jmin;
      for (j = jmin; j <= jmax; ++j) sum -= u[j] * tmp[ju[mu + j]];
    }

    tmp[k] = sum;
    z[c[k]] = sum;
    --k;
  } /* end of for i */

  /* exiting normally */
  *flag_ = 0;
  return 0;

} /* nnfc_ */

/* end */
