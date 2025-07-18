#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod demo {
pub mod Bench;
pub mod Buoyancy;
pub mod Chains;
pub mod ChipmunkDebugDraw;
pub mod ChipmunkDemo;
pub mod ChipmunkDemoTextSupport;
pub mod ContactGraph;
pub mod Convex;
pub mod Crane;
pub mod Joints;
pub mod LogoSmash;
pub mod OneWay;
pub mod Planet;
pub mod Player;
pub mod Plink;
pub mod Pump;
pub mod PyramidStack;
pub mod PyramidTopple;
pub mod Query;
pub mod Shatter;
pub mod Slice;
pub mod Springies;
pub mod Sticky;
pub mod Tank;
pub mod TheoJansen;
pub mod Tumble;
pub mod Unicycle;
pub mod sokol {
pub mod sokol;
} // mod sokol
} // mod demo
pub mod src {
pub mod chipmunk;
pub mod cpArbiter;
pub mod cpArray;
pub mod cpBBTree;
pub mod cpBody;
pub mod cpCollision;
pub mod cpConstraint;
pub mod cpDampedRotarySpring;
pub mod cpDampedSpring;
pub mod cpGearJoint;
pub mod cpGrooveJoint;
pub mod cpHashSet;
pub mod cpHastySpace;
pub mod cpMarch;
pub mod cpPinJoint;
pub mod cpPivotJoint;
pub mod cpPolyShape;
pub mod cpPolyline;
pub mod cpRatchetJoint;
pub mod cpRobust;
pub mod cpRotaryLimitJoint;
pub mod cpShape;
pub mod cpSimpleMotor;
pub mod cpSlideJoint;
pub mod cpSpace;
pub mod cpSpaceComponent;
pub mod cpSpaceDebug;
pub mod cpSpaceHash;
pub mod cpSpaceQuery;
pub mod cpSpaceStep;
pub mod cpSpatialIndex;
pub mod cpSweep1D;
} // mod src
