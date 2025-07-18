use ::libc;
extern "C" {
    pub type MemoryContextData;
    pub type dsa_area;
    pub type AttrMissing;
    pub type PgStat_TableStatus;
    pub type FdwRoutine;
    pub type IndexAmRoutine;
    pub type TableAmRoutine;
    pub type PartitionDescData;
    pub type PartitionKeyData;
    pub type RowSecurityDesc;
    pub type _MdfdVec;
    pub type QueryEnvironment;
    pub type JitInstrumentation;
    pub type JitContext;
    pub type PartitionDirectoryData;
    pub type CopyMultiInsertBuffer;
    pub type PartitionRoutingInfo;
    pub type SharedJitInstrumentation;
    pub type ExprEvalStep;
    pub type Tuplestorestate;
    pub type _SPI_plan;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pg_snprintf(
        str: *mut libc::c_char,
        count: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pfree(pointer: *mut libc::c_void);
    fn palloc(size: Size) -> *mut libc::c_void;
    fn elog_finish(elevel: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn elog_start(
        filename: *const libc::c_char,
        lineno: libc::c_int,
        funcname: *const libc::c_char,
    );
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn RelationForgetRelation(rid: Oid);
    fn initStringInfo(str: StringInfo);
    fn resetStringInfo(str: StringInfo);
    fn appendStringInfo(str: StringInfo, fmt: *const libc::c_char, _: ...);
    fn appendStringInfoString(str: StringInfo, s: *const libc::c_char);
    fn index_close(relation: Relation, lockmode: LOCKMODE);
    fn index_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
    fn TransactionIdFollows(id1: TransactionId, id2: TransactionId) -> bool;
    fn CommandCounterIncrement();
    fn deleteDependencyRecordsFor(
        classId: Oid,
        objectId: Oid,
        skipExtensionDeps: bool,
    ) -> libc::c_long;
    fn recordDependencyOn(
        depender: *const ObjectAddress,
        referenced: *const ObjectAddress,
        behavior: DependencyType,
    );
    fn construct_array(
        elems: *mut Datum,
        nelems: libc::c_int,
        elmtype: Oid,
        elmlen: libc::c_int,
        elmbyval: bool,
        elmalign: libc::c_char,
    ) -> *mut ArrayType;
    fn construct_empty_array(elmtype: Oid) -> *mut ArrayType;
    fn CatalogOpenIndexes(heapRel: Relation) -> CatalogIndexState;
    fn CatalogCloseIndexes(indstate: CatalogIndexState);
    fn CatalogTupleUpdateWithInfo(
        heapRel: Relation,
        otid: ItemPointer,
        tup: HeapTuple,
        indstate: CatalogIndexState,
    );
    fn RelationIsVisible(relid: Oid) -> bool;
    fn OpclassnameGetOpcid(amid: Oid, opcname: *const libc::c_char) -> Oid;
    fn table_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
    fn table_close(relation: Relation, lockmode: LOCKMODE);
    fn find_all_inheritors(
        parentrelId: Oid,
        lockmode: LOCKMODE,
        parents: *mut *mut List,
    ) -> *mut List;
    fn ATExecChangeOwner(
        relationOid: Oid,
        newOwnerId: Oid,
        recursing: bool,
        lockmode: LOCKMODE,
    );
    fn RenameRelationInternal(
        myrelid: Oid,
        newrelname: *const libc::c_char,
        is_internal: bool,
        is_index: bool,
    );
    fn heap_freetuple(htup: HeapTuple);
    fn superuser() -> bool;
    fn LockRelationOid(relid: Oid, lockmode: LOCKMODE);
    fn quote_qualified_identifier(
        qualifier: *const libc::c_char,
        ident: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn cstring_to_text(s: *const libc::c_char) -> *mut text;
    fn quote_identifier(ident: *const libc::c_char) -> *const libc::c_char;
    fn GetConfigOptionByName(
        name: *const libc::c_char,
        varname: *mut *const libc::c_char,
        missing_ok: bool,
    ) -> *mut libc::c_char;
    fn get_opfamily_member(
        opfamily: Oid,
        lefttype: Oid,
        righttype: Oid,
        strategy: int16,
    ) -> Oid;
    fn get_opname(opno: Oid) -> *mut libc::c_char;
    fn get_rel_name(relid: Oid) -> *mut libc::c_char;
    fn get_rel_namespace(relid: Oid) -> Oid;
    fn get_namespace_name(nspid: Oid) -> *mut libc::c_char;
    fn SearchSysCache(
        cacheId: libc::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> HeapTuple;
    fn ReleaseSysCache(tuple: HeapTuple);
    fn SearchSysCacheCopy(
        cacheId: libc::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> HeapTuple;
    fn SearchSysCacheExists(
        cacheId: libc::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> bool;
    static mut SPI_processed: uint64;
    static mut SPI_tuptable: *mut SPITupleTable;
    static mut SPI_result: libc::c_int;
    fn SPI_connect() -> libc::c_int;
    fn SPI_finish() -> libc::c_int;
    fn SPI_prepare(
        src: *const libc::c_char,
        nargs: libc::c_int,
        argtypes: *mut Oid,
    ) -> SPIPlanPtr;
    fn SPI_returntuple(tuple: HeapTuple, tupdesc: TupleDesc) -> HeapTupleHeader;
    fn SPI_getvalue(
        tuple: HeapTuple,
        tupdesc: TupleDesc,
        fnumber: libc::c_int,
    ) -> *mut libc::c_char;
    fn SPI_getbinval(
        tuple: HeapTuple,
        tupdesc: TupleDesc,
        fnumber: libc::c_int,
        isnull: *mut bool,
    ) -> Datum;
    fn SPI_gettypeid(tupdesc: TupleDesc, fnumber: libc::c_int) -> Oid;
    fn SPI_freetuptable(tuptable: *mut SPITupleTable);
    fn execute(expected: libc::c_int, sql: *const libc::c_char);
    fn execute_plan(
        expected: libc::c_int,
        plan: SPIPlanPtr,
        values: *mut Datum,
        nulls: *const libc::c_char,
    );
    fn execute_with_format(expected: libc::c_int, format: *const libc::c_char, _: ...);
    fn execute_with_args(
        expected: libc::c_int,
        src: *const libc::c_char,
        nargs: libc::c_int,
        argtypes: *mut Oid,
        values: *mut Datum,
        nulls: *const bool,
    );
    fn pg_get_indexdef_string(indexrelid: Oid) -> *mut libc::c_char;
}
pub type Oid = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintptr_t = libc::c_ulong;
pub type Pointer = *mut libc::c_char;
pub type int16 = libc::c_short;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
pub type bits8 = uint8;
pub type int64 = libc::c_long;
pub type uint64 = libc::c_ulong;
pub type Size = size_t;
pub type Index = libc::c_uint;
pub type float4 = libc::c_float;
pub type regproc = Oid;
pub type RegProcedure = regproc;
pub type TransactionId = uint32;
pub type SubTransactionId = uint32;
pub type CommandId = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varlena {
    pub vl_len_: [libc::c_char; 4],
    pub vl_dat: [libc::c_char; 0],
}
pub type bytea = varlena;
pub type text = varlena;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int2vector {
    pub vl_len_: int32,
    pub ndim: libc::c_int,
    pub dataoffset: int32,
    pub elemtype: Oid,
    pub dim1: libc::c_int,
    pub lbound1: libc::c_int,
    pub values: [int16; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameData {
    pub data: [libc::c_char; 64],
}
pub type NameData = nameData;
pub type Name = *mut NameData;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type MemoryContext = *mut MemoryContextData;
pub type Datum = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
pub type ScanDirection = libc::c_int;
pub const ForwardScanDirection: ScanDirection = 1;
pub const NoMovementScanDirection: ScanDirection = 0;
pub const BackwardScanDirection: ScanDirection = -1;
pub type AttrNumber = int16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
}
pub type NodeTag = libc::c_uint;
pub const T_SupportRequestIndexCondition: NodeTag = 416;
pub const T_SupportRequestRows: NodeTag = 415;
pub const T_SupportRequestCost: NodeTag = 414;
pub const T_SupportRequestSelectivity: NodeTag = 413;
pub const T_SupportRequestSimplify: NodeTag = 412;
pub const T_CallContext: NodeTag = 411;
pub const T_ForeignKeyCacheInfo: NodeTag = 410;
pub const T_TsmRoutine: NodeTag = 409;
pub const T_TableAmRoutine: NodeTag = 408;
pub const T_IndexAmRoutine: NodeTag = 407;
pub const T_FdwRoutine: NodeTag = 406;
pub const T_InlineCodeBlock: NodeTag = 405;
pub const T_TIDBitmap: NodeTag = 404;
pub const T_WindowObjectData: NodeTag = 403;
pub const T_ReturnSetInfo: NodeTag = 402;
pub const T_EventTriggerData: NodeTag = 401;
pub const T_TriggerData: NodeTag = 400;
pub const T_SQLCmd: NodeTag = 399;
pub const T_TimeLineHistoryCmd: NodeTag = 398;
pub const T_StartReplicationCmd: NodeTag = 397;
pub const T_DropReplicationSlotCmd: NodeTag = 396;
pub const T_CreateReplicationSlotCmd: NodeTag = 395;
pub const T_BaseBackupCmd: NodeTag = 394;
pub const T_IdentifySystemCmd: NodeTag = 393;
pub const T_VacuumRelation: NodeTag = 392;
pub const T_PartitionCmd: NodeTag = 391;
pub const T_PartitionRangeDatum: NodeTag = 390;
pub const T_PartitionBoundSpec: NodeTag = 389;
pub const T_PartitionSpec: NodeTag = 388;
pub const T_PartitionElem: NodeTag = 387;
pub const T_TriggerTransition: NodeTag = 386;
pub const T_RoleSpec: NodeTag = 385;
pub const T_CommonTableExpr: NodeTag = 384;
pub const T_OnConflictClause: NodeTag = 383;
pub const T_InferClause: NodeTag = 382;
pub const T_WithClause: NodeTag = 381;
pub const T_XmlSerialize: NodeTag = 380;
pub const T_RowMarkClause: NodeTag = 379;
pub const T_LockingClause: NodeTag = 378;
pub const T_FunctionParameter: NodeTag = 377;
pub const T_TableLikeClause: NodeTag = 376;
pub const T_CreateOpClassItem: NodeTag = 375;
pub const T_AccessPriv: NodeTag = 374;
pub const T_ObjectWithArgs: NodeTag = 373;
pub const T_WindowClause: NodeTag = 372;
pub const T_GroupingSet: NodeTag = 371;
pub const T_SortGroupClause: NodeTag = 370;
pub const T_WithCheckOption: NodeTag = 369;
pub const T_TableSampleClause: NodeTag = 368;
pub const T_RangeTblFunction: NodeTag = 367;
pub const T_RangeTblEntry: NodeTag = 366;
pub const T_DefElem: NodeTag = 365;
pub const T_Constraint: NodeTag = 364;
pub const T_IndexElem: NodeTag = 363;
pub const T_ColumnDef: NodeTag = 362;
pub const T_TypeName: NodeTag = 361;
pub const T_RangeTableFuncCol: NodeTag = 360;
pub const T_RangeTableFunc: NodeTag = 359;
pub const T_RangeTableSample: NodeTag = 358;
pub const T_RangeFunction: NodeTag = 357;
pub const T_RangeSubselect: NodeTag = 356;
pub const T_WindowDef: NodeTag = 355;
pub const T_SortBy: NodeTag = 354;
pub const T_CollateClause: NodeTag = 353;
pub const T_TypeCast: NodeTag = 352;
pub const T_MultiAssignRef: NodeTag = 351;
pub const T_ResTarget: NodeTag = 350;
pub const T_A_ArrayExpr: NodeTag = 349;
pub const T_A_Indirection: NodeTag = 348;
pub const T_A_Indices: NodeTag = 347;
pub const T_A_Star: NodeTag = 346;
pub const T_FuncCall: NodeTag = 345;
pub const T_A_Const: NodeTag = 344;
pub const T_ParamRef: NodeTag = 343;
pub const T_ColumnRef: NodeTag = 342;
pub const T_A_Expr: NodeTag = 341;
pub const T_CallStmt: NodeTag = 340;
pub const T_AlterCollationStmt: NodeTag = 339;
pub const T_CreateStatsStmt: NodeTag = 338;
pub const T_DropSubscriptionStmt: NodeTag = 337;
pub const T_AlterSubscriptionStmt: NodeTag = 336;
pub const T_CreateSubscriptionStmt: NodeTag = 335;
pub const T_AlterPublicationStmt: NodeTag = 334;
pub const T_CreatePublicationStmt: NodeTag = 333;
pub const T_CreateAmStmt: NodeTag = 332;
pub const T_CreateTransformStmt: NodeTag = 331;
pub const T_AlterPolicyStmt: NodeTag = 330;
pub const T_CreatePolicyStmt: NodeTag = 329;
pub const T_AlterSystemStmt: NodeTag = 328;
pub const T_ReplicaIdentityStmt: NodeTag = 327;
pub const T_RefreshMatViewStmt: NodeTag = 326;
pub const T_AlterEventTrigStmt: NodeTag = 325;
pub const T_CreateEventTrigStmt: NodeTag = 324;
pub const T_AlterExtensionContentsStmt: NodeTag = 323;
pub const T_AlterExtensionStmt: NodeTag = 322;
pub const T_CreateExtensionStmt: NodeTag = 321;
pub const T_ImportForeignSchemaStmt: NodeTag = 320;
pub const T_CreateForeignTableStmt: NodeTag = 319;
pub const T_SecLabelStmt: NodeTag = 318;
pub const T_AlterTableMoveAllStmt: NodeTag = 317;
pub const T_AlterTableSpaceOptionsStmt: NodeTag = 316;
pub const T_DropUserMappingStmt: NodeTag = 315;
pub const T_AlterUserMappingStmt: NodeTag = 314;
pub const T_CreateUserMappingStmt: NodeTag = 313;
pub const T_AlterForeignServerStmt: NodeTag = 312;
pub const T_CreateForeignServerStmt: NodeTag = 311;
pub const T_AlterFdwStmt: NodeTag = 310;
pub const T_CreateFdwStmt: NodeTag = 309;
pub const T_AlterTSConfigurationStmt: NodeTag = 308;
pub const T_AlterTSDictionaryStmt: NodeTag = 307;
pub const T_AlterEnumStmt: NodeTag = 306;
pub const T_CreateRangeStmt: NodeTag = 305;
pub const T_CreateEnumStmt: NodeTag = 304;
pub const T_CompositeTypeStmt: NodeTag = 303;
pub const T_ReassignOwnedStmt: NodeTag = 302;
pub const T_DropOwnedStmt: NodeTag = 301;
pub const T_AlterOperatorStmt: NodeTag = 300;
pub const T_AlterOwnerStmt: NodeTag = 299;
pub const T_AlterObjectSchemaStmt: NodeTag = 298;
pub const T_AlterObjectDependsStmt: NodeTag = 297;
pub const T_DropTableSpaceStmt: NodeTag = 296;
pub const T_CreateTableSpaceStmt: NodeTag = 295;
pub const T_DeclareCursorStmt: NodeTag = 294;
pub const T_DeallocateStmt: NodeTag = 293;
pub const T_ExecuteStmt: NodeTag = 292;
pub const T_PrepareStmt: NodeTag = 291;
pub const T_AlterOpFamilyStmt: NodeTag = 290;
pub const T_CreateOpFamilyStmt: NodeTag = 289;
pub const T_CreateOpClassStmt: NodeTag = 288;
pub const T_CreateCastStmt: NodeTag = 287;
pub const T_CreateConversionStmt: NodeTag = 286;
pub const T_AlterRoleSetStmt: NodeTag = 285;
pub const T_AlterDatabaseSetStmt: NodeTag = 284;
pub const T_AlterDatabaseStmt: NodeTag = 283;
pub const T_CreateSchemaStmt: NodeTag = 282;
pub const T_CheckPointStmt: NodeTag = 281;
pub const T_ReindexStmt: NodeTag = 280;
pub const T_ConstraintsSetStmt: NodeTag = 279;
pub const T_LockStmt: NodeTag = 278;
pub const T_DropRoleStmt: NodeTag = 277;
pub const T_AlterRoleStmt: NodeTag = 276;
pub const T_CreateRoleStmt: NodeTag = 275;
pub const T_CreatePLangStmt: NodeTag = 274;
pub const T_CreateTrigStmt: NodeTag = 273;
pub const T_DiscardStmt: NodeTag = 272;
pub const T_VariableShowStmt: NodeTag = 271;
pub const T_VariableSetStmt: NodeTag = 270;
pub const T_AlterSeqStmt: NodeTag = 269;
pub const T_CreateSeqStmt: NodeTag = 268;
pub const T_CreateTableAsStmt: NodeTag = 267;
pub const T_ExplainStmt: NodeTag = 266;
pub const T_VacuumStmt: NodeTag = 265;
pub const T_DropdbStmt: NodeTag = 264;
pub const T_CreatedbStmt: NodeTag = 263;
pub const T_CreateDomainStmt: NodeTag = 262;
pub const T_LoadStmt: NodeTag = 261;
pub const T_ViewStmt: NodeTag = 260;
pub const T_TransactionStmt: NodeTag = 259;
pub const T_UnlistenStmt: NodeTag = 258;
pub const T_ListenStmt: NodeTag = 257;
pub const T_NotifyStmt: NodeTag = 256;
pub const T_RuleStmt: NodeTag = 255;
pub const T_RenameStmt: NodeTag = 254;
pub const T_DoStmt: NodeTag = 253;
pub const T_AlterFunctionStmt: NodeTag = 252;
pub const T_CreateFunctionStmt: NodeTag = 251;
pub const T_IndexStmt: NodeTag = 250;
pub const T_FetchStmt: NodeTag = 249;
pub const T_CommentStmt: NodeTag = 248;
pub const T_TruncateStmt: NodeTag = 247;
pub const T_DropStmt: NodeTag = 246;
pub const T_DefineStmt: NodeTag = 245;
pub const T_CreateStmt: NodeTag = 244;
pub const T_CopyStmt: NodeTag = 243;
pub const T_ClusterStmt: NodeTag = 242;
pub const T_ClosePortalStmt: NodeTag = 241;
pub const T_AlterDefaultPrivilegesStmt: NodeTag = 240;
pub const T_GrantRoleStmt: NodeTag = 239;
pub const T_GrantStmt: NodeTag = 238;
pub const T_SetOperationStmt: NodeTag = 237;
pub const T_AlterDomainStmt: NodeTag = 236;
pub const T_AlterTableCmd: NodeTag = 235;
pub const T_AlterTableStmt: NodeTag = 234;
pub const T_SelectStmt: NodeTag = 233;
pub const T_UpdateStmt: NodeTag = 232;
pub const T_DeleteStmt: NodeTag = 231;
pub const T_InsertStmt: NodeTag = 230;
pub const T_PlannedStmt: NodeTag = 229;
pub const T_Query: NodeTag = 228;
pub const T_RawStmt: NodeTag = 227;
pub const T_ExtensibleNode: NodeTag = 226;
pub const T_OidList: NodeTag = 225;
pub const T_IntList: NodeTag = 224;
pub const T_List: NodeTag = 223;
pub const T_Null: NodeTag = 222;
pub const T_BitString: NodeTag = 221;
pub const T_String: NodeTag = 220;
pub const T_Float: NodeTag = 219;
pub const T_Integer: NodeTag = 218;
pub const T_Value: NodeTag = 217;
pub const T_GenerationContext: NodeTag = 216;
pub const T_SlabContext: NodeTag = 215;
pub const T_AllocSetContext: NodeTag = 214;
pub const T_MemoryContext: NodeTag = 213;
pub const T_StatisticExtInfo: NodeTag = 212;
pub const T_GroupingSetData: NodeTag = 211;
pub const T_RollupData: NodeTag = 210;
pub const T_PlannerParamItem: NodeTag = 209;
pub const T_MinMaxAggInfo: NodeTag = 208;
pub const T_PlaceHolderInfo: NodeTag = 207;
pub const T_AppendRelInfo: NodeTag = 206;
pub const T_SpecialJoinInfo: NodeTag = 205;
pub const T_PlaceHolderVar: NodeTag = 204;
pub const T_IndexClause: NodeTag = 203;
pub const T_RestrictInfo: NodeTag = 202;
pub const T_PathTarget: NodeTag = 201;
pub const T_PathKey: NodeTag = 200;
pub const T_EquivalenceMember: NodeTag = 199;
pub const T_EquivalenceClass: NodeTag = 198;
pub const T_LimitPath: NodeTag = 197;
pub const T_ModifyTablePath: NodeTag = 196;
pub const T_LockRowsPath: NodeTag = 195;
pub const T_RecursiveUnionPath: NodeTag = 194;
pub const T_SetOpPath: NodeTag = 193;
pub const T_WindowAggPath: NodeTag = 192;
pub const T_MinMaxAggPath: NodeTag = 191;
pub const T_GroupingSetsPath: NodeTag = 190;
pub const T_AggPath: NodeTag = 189;
pub const T_UpperUniquePath: NodeTag = 188;
pub const T_GroupPath: NodeTag = 187;
pub const T_SortPath: NodeTag = 186;
pub const T_ProjectSetPath: NodeTag = 185;
pub const T_ProjectionPath: NodeTag = 184;
pub const T_GatherMergePath: NodeTag = 183;
pub const T_GatherPath: NodeTag = 182;
pub const T_UniquePath: NodeTag = 181;
pub const T_MaterialPath: NodeTag = 180;
pub const T_GroupResultPath: NodeTag = 179;
pub const T_MergeAppendPath: NodeTag = 178;
pub const T_AppendPath: NodeTag = 177;
pub const T_HashPath: NodeTag = 176;
pub const T_MergePath: NodeTag = 175;
pub const T_NestPath: NodeTag = 174;
pub const T_CustomPath: NodeTag = 173;
pub const T_ForeignPath: NodeTag = 172;
pub const T_SubqueryScanPath: NodeTag = 171;
pub const T_TidPath: NodeTag = 170;
pub const T_BitmapOrPath: NodeTag = 169;
pub const T_BitmapAndPath: NodeTag = 168;
pub const T_BitmapHeapPath: NodeTag = 167;
pub const T_IndexPath: NodeTag = 166;
pub const T_Path: NodeTag = 165;
pub const T_ParamPathInfo: NodeTag = 164;
pub const T_ForeignKeyOptInfo: NodeTag = 163;
pub const T_IndexOptInfo: NodeTag = 162;
pub const T_RelOptInfo: NodeTag = 161;
pub const T_PlannerGlobal: NodeTag = 160;
pub const T_PlannerInfo: NodeTag = 159;
pub const T_DomainConstraintState: NodeTag = 158;
pub const T_AlternativeSubPlanState: NodeTag = 157;
pub const T_SubPlanState: NodeTag = 156;
pub const T_SetExprState: NodeTag = 155;
pub const T_WindowFuncExprState: NodeTag = 154;
pub const T_AggrefExprState: NodeTag = 153;
pub const T_ExprState: NodeTag = 152;
pub const T_IntoClause: NodeTag = 151;
pub const T_OnConflictExpr: NodeTag = 150;
pub const T_FromExpr: NodeTag = 149;
pub const T_JoinExpr: NodeTag = 148;
pub const T_RangeTblRef: NodeTag = 147;
pub const T_TargetEntry: NodeTag = 146;
pub const T_InferenceElem: NodeTag = 145;
pub const T_NextValueExpr: NodeTag = 144;
pub const T_CurrentOfExpr: NodeTag = 143;
pub const T_SetToDefault: NodeTag = 142;
pub const T_CoerceToDomainValue: NodeTag = 141;
pub const T_CoerceToDomain: NodeTag = 140;
pub const T_BooleanTest: NodeTag = 139;
pub const T_NullTest: NodeTag = 138;
pub const T_XmlExpr: NodeTag = 137;
pub const T_SQLValueFunction: NodeTag = 136;
pub const T_MinMaxExpr: NodeTag = 135;
pub const T_CoalesceExpr: NodeTag = 134;
pub const T_RowCompareExpr: NodeTag = 133;
pub const T_RowExpr: NodeTag = 132;
pub const T_ArrayExpr: NodeTag = 131;
pub const T_CaseTestExpr: NodeTag = 130;
pub const T_CaseWhen: NodeTag = 129;
pub const T_CaseExpr: NodeTag = 128;
pub const T_CollateExpr: NodeTag = 127;
pub const T_ConvertRowtypeExpr: NodeTag = 126;
pub const T_ArrayCoerceExpr: NodeTag = 125;
pub const T_CoerceViaIO: NodeTag = 124;
pub const T_RelabelType: NodeTag = 123;
pub const T_FieldStore: NodeTag = 122;
pub const T_FieldSelect: NodeTag = 121;
pub const T_AlternativeSubPlan: NodeTag = 120;
pub const T_SubPlan: NodeTag = 119;
pub const T_SubLink: NodeTag = 118;
pub const T_BoolExpr: NodeTag = 117;
pub const T_ScalarArrayOpExpr: NodeTag = 116;
pub const T_NullIfExpr: NodeTag = 115;
pub const T_DistinctExpr: NodeTag = 114;
pub const T_OpExpr: NodeTag = 113;
pub const T_NamedArgExpr: NodeTag = 112;
pub const T_FuncExpr: NodeTag = 111;
pub const T_SubscriptingRef: NodeTag = 110;
pub const T_WindowFunc: NodeTag = 109;
pub const T_GroupingFunc: NodeTag = 108;
pub const T_Aggref: NodeTag = 107;
pub const T_Param: NodeTag = 106;
pub const T_Const: NodeTag = 105;
pub const T_Var: NodeTag = 104;
pub const T_Expr: NodeTag = 103;
pub const T_TableFunc: NodeTag = 102;
pub const T_RangeVar: NodeTag = 101;
pub const T_Alias: NodeTag = 100;
pub const T_LimitState: NodeTag = 99;
pub const T_LockRowsState: NodeTag = 98;
pub const T_SetOpState: NodeTag = 97;
pub const T_HashState: NodeTag = 96;
pub const T_GatherMergeState: NodeTag = 95;
pub const T_GatherState: NodeTag = 94;
pub const T_UniqueState: NodeTag = 93;
pub const T_WindowAggState: NodeTag = 92;
pub const T_AggState: NodeTag = 91;
pub const T_GroupState: NodeTag = 90;
pub const T_SortState: NodeTag = 89;
pub const T_MaterialState: NodeTag = 88;
pub const T_HashJoinState: NodeTag = 87;
pub const T_MergeJoinState: NodeTag = 86;
pub const T_NestLoopState: NodeTag = 85;
pub const T_JoinState: NodeTag = 84;
pub const T_CustomScanState: NodeTag = 83;
pub const T_ForeignScanState: NodeTag = 82;
pub const T_WorkTableScanState: NodeTag = 81;
pub const T_NamedTuplestoreScanState: NodeTag = 80;
pub const T_CteScanState: NodeTag = 79;
pub const T_ValuesScanState: NodeTag = 78;
pub const T_TableFuncScanState: NodeTag = 77;
pub const T_FunctionScanState: NodeTag = 76;
pub const T_SubqueryScanState: NodeTag = 75;
pub const T_TidScanState: NodeTag = 74;
pub const T_BitmapHeapScanState: NodeTag = 73;
pub const T_BitmapIndexScanState: NodeTag = 72;
pub const T_IndexOnlyScanState: NodeTag = 71;
pub const T_IndexScanState: NodeTag = 70;
pub const T_SampleScanState: NodeTag = 69;
pub const T_SeqScanState: NodeTag = 68;
pub const T_ScanState: NodeTag = 67;
pub const T_BitmapOrState: NodeTag = 66;
pub const T_BitmapAndState: NodeTag = 65;
pub const T_RecursiveUnionState: NodeTag = 64;
pub const T_MergeAppendState: NodeTag = 63;
pub const T_AppendState: NodeTag = 62;
pub const T_ModifyTableState: NodeTag = 61;
pub const T_ProjectSetState: NodeTag = 60;
pub const T_ResultState: NodeTag = 59;
pub const T_PlanState: NodeTag = 58;
pub const T_PlanInvalItem: NodeTag = 57;
pub const T_PartitionPruneStepCombine: NodeTag = 56;
pub const T_PartitionPruneStepOp: NodeTag = 55;
pub const T_PartitionedRelPruneInfo: NodeTag = 54;
pub const T_PartitionPruneInfo: NodeTag = 53;
pub const T_PlanRowMark: NodeTag = 52;
pub const T_NestLoopParam: NodeTag = 51;
pub const T_Limit: NodeTag = 50;
pub const T_LockRows: NodeTag = 49;
pub const T_SetOp: NodeTag = 48;
pub const T_Hash: NodeTag = 47;
pub const T_GatherMerge: NodeTag = 46;
pub const T_Gather: NodeTag = 45;
pub const T_Unique: NodeTag = 44;
pub const T_WindowAgg: NodeTag = 43;
pub const T_Agg: NodeTag = 42;
pub const T_Group: NodeTag = 41;
pub const T_Sort: NodeTag = 40;
pub const T_Material: NodeTag = 39;
pub const T_HashJoin: NodeTag = 38;
pub const T_MergeJoin: NodeTag = 37;
pub const T_NestLoop: NodeTag = 36;
pub const T_Join: NodeTag = 35;
pub const T_CustomScan: NodeTag = 34;
pub const T_ForeignScan: NodeTag = 33;
pub const T_WorkTableScan: NodeTag = 32;
pub const T_NamedTuplestoreScan: NodeTag = 31;
pub const T_CteScan: NodeTag = 30;
pub const T_TableFuncScan: NodeTag = 29;
pub const T_ValuesScan: NodeTag = 28;
pub const T_FunctionScan: NodeTag = 27;
pub const T_SubqueryScan: NodeTag = 26;
pub const T_TidScan: NodeTag = 25;
pub const T_BitmapHeapScan: NodeTag = 24;
pub const T_BitmapIndexScan: NodeTag = 23;
pub const T_IndexOnlyScan: NodeTag = 22;
pub const T_IndexScan: NodeTag = 21;
pub const T_SampleScan: NodeTag = 20;
pub const T_SeqScan: NodeTag = 19;
pub const T_Scan: NodeTag = 18;
pub const T_BitmapOr: NodeTag = 17;
pub const T_BitmapAnd: NodeTag = 16;
pub const T_RecursiveUnion: NodeTag = 15;
pub const T_MergeAppend: NodeTag = 14;
pub const T_Append: NodeTag = 13;
pub const T_ModifyTable: NodeTag = 12;
pub const T_ProjectSet: NodeTag = 11;
pub const T_Result: NodeTag = 10;
pub const T_Plan: NodeTag = 9;
pub const T_TupleTableSlot: NodeTag = 8;
pub const T_EState: NodeTag = 7;
pub const T_ResultRelInfo: NodeTag = 6;
pub const T_OnConflictSetState: NodeTag = 5;
pub const T_JunkFilter: NodeTag = 4;
pub const T_ProjectionInfo: NodeTag = 3;
pub const T_ExprContext: NodeTag = 2;
pub const T_IndexInfo: NodeTag = 1;
pub const T_Invalid: NodeTag = 0;
pub type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub type_0: NodeTag,
    pub length: libc::c_int,
    pub head: *mut ListCell,
    pub tail: *mut ListCell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListCell {
    pub data: C2RustUnnamed_0,
    pub next: *mut ListCell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ptr_value: *mut libc::c_void,
    pub int_value: libc::c_int,
    pub oid_value: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringInfoData {
    pub data: *mut libc::c_char,
    pub len: libc::c_int,
    pub maxlen: libc::c_int,
    pub cursor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool,
    pub nargs: libc::c_short,
    pub args: [NullableDatum; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FmgrInfo {
    pub fn_addr: PGFunction,
    pub fn_oid: Oid,
    pub fn_nargs: libc::c_short,
    pub fn_strict: bool,
    pub fn_retset: bool,
    pub fn_stats: libc::c_uchar,
    pub fn_extra: *mut libc::c_void,
    pub fn_mcxt: MemoryContext,
    pub fn_expr: fmNodePtr,
}
pub type PGFunction = Option::<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pg_finfo_record {
    pub api_version: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pg_magic_struct {
    pub len: libc::c_int,
    pub version: libc::c_int,
    pub funcmaxargs: libc::c_int,
    pub indexmaxkeys: libc::c_int,
    pub namedatalen: libc::c_int,
    pub float4byval: libc::c_int,
    pub float8byval: libc::c_int,
}
pub type BlockNumber = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockIdData {
    pub bi_hi: uint16,
    pub bi_lo: uint16,
}
pub type OffsetNumber = uint16;
#[derive(Copy, Clone)]
#[repr(C, align(2))]
pub struct ItemPointerData(pub ItemPointerData_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ItemPointerData_Inner {
    pub ip_blkid: BlockIdData,
    pub ip_posid: OffsetNumber,
}
#[allow(dead_code, non_upper_case_globals)]
const ItemPointerData_PADDING: usize = ::std::mem::size_of::<ItemPointerData>()
    - ::std::mem::size_of::<ItemPointerData_Inner>();
pub type ItemPointer = *mut ItemPointerData;
pub type LOCKMODE = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: int32,
    pub attlen: int16,
    pub attnum: int16,
    pub attndims: int32,
    pub attcacheoff: int32,
    pub atttypmod: int32,
    pub attbyval: bool,
    pub attstorage: libc::c_char,
    pub attalign: libc::c_char,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub atthasmissing: bool,
    pub attidentity: libc::c_char,
    pub attgenerated: libc::c_char,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: int32,
    pub attcollation: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
pub type bitmapword = uint64;
pub type Cost = libc::c_double;
pub type CmdType = libc::c_uint;
pub const CMD_NOTHING: CmdType = 6;
pub const CMD_UTILITY: CmdType = 5;
pub const CMD_DELETE: CmdType = 4;
pub const CMD_INSERT: CmdType = 3;
pub const CMD_UPDATE: CmdType = 2;
pub const CMD_SELECT: CmdType = 1;
pub const CMD_UNKNOWN: CmdType = 0;
pub type JoinType = libc::c_uint;
pub const JOIN_UNIQUE_INNER: JoinType = 7;
pub const JOIN_UNIQUE_OUTER: JoinType = 6;
pub const JOIN_ANTI: JoinType = 5;
pub const JOIN_SEMI: JoinType = 4;
pub const JOIN_RIGHT: JoinType = 3;
pub const JOIN_FULL: JoinType = 2;
pub const JOIN_LEFT: JoinType = 1;
pub const JOIN_INNER: JoinType = 0;
pub type OnConflictAction = libc::c_uint;
pub const ONCONFLICT_UPDATE: OnConflictAction = 2;
pub const ONCONFLICT_NOTHING: OnConflictAction = 1;
pub const ONCONFLICT_NONE: OnConflictAction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttrDefault {
    pub adnum: AttrNumber,
    pub adbin: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConstrCheck {
    pub ccname: *mut libc::c_char,
    pub ccbin: *mut libc::c_char,
    pub ccvalid: bool,
    pub ccnoinherit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub missing: *mut AttrMissing,
    pub num_defval: uint16,
    pub num_check: uint16,
    pub has_not_null: bool,
    pub has_generated_stored: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleDescData {
    pub natts: libc::c_int,
    pub tdtypeid: Oid,
    pub tdtypmod: int32,
    pub tdrefcount: libc::c_int,
    pub constr: *mut TupleConstr,
    pub attrs: [FormData_pg_attribute; 0],
}
pub type TupleDesc = *mut TupleDescData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelationData {
    pub rd_node: RelFileNode,
    pub rd_smgr: SMgrRelation,
    pub rd_refcnt: libc::c_int,
    pub rd_backend: BackendId,
    pub rd_islocaltemp: bool,
    pub rd_isnailed: bool,
    pub rd_isvalid: bool,
    pub rd_indexvalid: bool,
    pub rd_statvalid: bool,
    pub rd_createSubid: SubTransactionId,
    pub rd_newRelfilenodeSubid: SubTransactionId,
    pub rd_rel: Form_pg_class,
    pub rd_att: TupleDesc,
    pub rd_id: Oid,
    pub rd_lockInfo: LockInfoData,
    pub rd_rules: *mut RuleLock,
    pub rd_rulescxt: MemoryContext,
    pub trigdesc: *mut TriggerDesc,
    pub rd_rsdesc: *mut RowSecurityDesc,
    pub rd_fkeylist: *mut List,
    pub rd_fkeyvalid: bool,
    pub rd_partkey: *mut PartitionKeyData,
    pub rd_partkeycxt: MemoryContext,
    pub rd_partdesc: *mut PartitionDescData,
    pub rd_pdcxt: MemoryContext,
    pub rd_partcheck: *mut List,
    pub rd_partcheckvalid: bool,
    pub rd_partcheckcxt: MemoryContext,
    pub rd_indexlist: *mut List,
    pub rd_pkindex: Oid,
    pub rd_replidindex: Oid,
    pub rd_statlist: *mut List,
    pub rd_indexattr: *mut Bitmapset,
    pub rd_keyattr: *mut Bitmapset,
    pub rd_pkattr: *mut Bitmapset,
    pub rd_idattr: *mut Bitmapset,
    pub rd_pubactions: *mut PublicationActions,
    pub rd_options: *mut bytea,
    pub rd_amhandler: Oid,
    pub rd_tableam: *const TableAmRoutine,
    pub rd_index: Form_pg_index,
    pub rd_indextuple: *mut HeapTupleData,
    pub rd_indexcxt: MemoryContext,
    pub rd_indam: *mut IndexAmRoutine,
    pub rd_opfamily: *mut Oid,
    pub rd_opcintype: *mut Oid,
    pub rd_support: *mut RegProcedure,
    pub rd_supportinfo: *mut FmgrInfo,
    pub rd_indoption: *mut int16,
    pub rd_indexprs: *mut List,
    pub rd_indpred: *mut List,
    pub rd_exclops: *mut Oid,
    pub rd_exclprocs: *mut Oid,
    pub rd_exclstrats: *mut uint16,
    pub rd_indcollation: *mut Oid,
    pub rd_amcache: *mut libc::c_void,
    pub rd_fdwroutine: *mut FdwRoutine,
    pub rd_toastoid: Oid,
    pub pgstat_info: *mut PgStat_TableStatus,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleData {
    pub t_len: uint32,
    pub t_self: ItemPointerData,
    pub t_tableOid: Oid,
    pub t_data: HeapTupleHeader,
}
pub type HeapTupleHeader = *mut HeapTupleHeaderData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleHeaderData {
    pub t_choice: C2RustUnnamed_1,
    pub t_ctid: ItemPointerData,
    pub t_infomask2: uint16,
    pub t_infomask: uint16,
    pub t_hoff: uint8,
    pub t_bits: [bits8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub t_heap: HeapTupleFields,
    pub t_datum: DatumTupleFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatumTupleFields {
    pub datum_len_: int32,
    pub datum_typmod: int32,
    pub datum_typeid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleFields {
    pub t_xmin: TransactionId,
    pub t_xmax: TransactionId,
    pub t_field3: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub t_cid: CommandId,
    pub t_xvac: TransactionId,
}
pub type Form_pg_index = *mut FormData_pg_index;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_index {
    pub indexrelid: Oid,
    pub indrelid: Oid,
    pub indnatts: int16,
    pub indnkeyatts: int16,
    pub indisunique: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisclustered: bool,
    pub indisvalid: bool,
    pub indcheckxmin: bool,
    pub indisready: bool,
    pub indislive: bool,
    pub indisreplident: bool,
    pub indkey: int2vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PublicationActions {
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerDesc {
    pub triggers: *mut Trigger,
    pub numtriggers: libc::c_int,
    pub trig_insert_before_row: bool,
    pub trig_insert_after_row: bool,
    pub trig_insert_instead_row: bool,
    pub trig_insert_before_statement: bool,
    pub trig_insert_after_statement: bool,
    pub trig_update_before_row: bool,
    pub trig_update_after_row: bool,
    pub trig_update_instead_row: bool,
    pub trig_update_before_statement: bool,
    pub trig_update_after_statement: bool,
    pub trig_delete_before_row: bool,
    pub trig_delete_after_row: bool,
    pub trig_delete_instead_row: bool,
    pub trig_delete_before_statement: bool,
    pub trig_delete_after_statement: bool,
    pub trig_truncate_before_statement: bool,
    pub trig_truncate_after_statement: bool,
    pub trig_insert_new_table: bool,
    pub trig_update_old_table: bool,
    pub trig_update_new_table: bool,
    pub trig_delete_old_table: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub tgoid: Oid,
    pub tgname: *mut libc::c_char,
    pub tgfoid: Oid,
    pub tgtype: int16,
    pub tgenabled: libc::c_char,
    pub tgisinternal: bool,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: int16,
    pub tgnattr: int16,
    pub tgattr: *mut int16,
    pub tgargs: *mut *mut libc::c_char,
    pub tgqual: *mut libc::c_char,
    pub tgoldtable: *mut libc::c_char,
    pub tgnewtable: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RuleLock {
    pub numLocks: libc::c_int,
    pub rules: *mut *mut RewriteRule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RewriteRule {
    pub ruleId: Oid,
    pub event: CmdType,
    pub qual: *mut Node,
    pub actions: *mut List,
    pub enabled: libc::c_char,
    pub isInstead: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LockInfoData {
    pub lockRelId: LockRelId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LockRelId {
    pub relId: Oid,
    pub dbId: Oid,
}
pub type Form_pg_class = *mut FormData_pg_class;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_class {
    pub oid: Oid,
    pub relname: NameData,
    pub relnamespace: Oid,
    pub reltype: Oid,
    pub reloftype: Oid,
    pub relowner: Oid,
    pub relam: Oid,
    pub relfilenode: Oid,
    pub reltablespace: Oid,
    pub relpages: int32,
    pub reltuples: float4,
    pub relallvisible: int32,
    pub reltoastrelid: Oid,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: libc::c_char,
    pub relkind: libc::c_char,
    pub relnatts: int16,
    pub relchecks: int16,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: libc::c_char,
    pub relispartition: bool,
    pub relrewrite: Oid,
    pub relfrozenxid: TransactionId,
    pub relminmxid: TransactionId,
}
pub type BackendId = libc::c_int;
pub type SMgrRelation = *mut SMgrRelationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SMgrRelationData {
    pub smgr_rnode: RelFileNodeBackend,
    pub smgr_owner: *mut *mut SMgrRelationData,
    pub smgr_targblock: BlockNumber,
    pub smgr_fsm_nblocks: BlockNumber,
    pub smgr_vm_nblocks: BlockNumber,
    pub smgr_which: libc::c_int,
    pub md_num_open_segs: [libc::c_int; 4],
    pub md_seg_fds: [*mut _MdfdVec; 4],
    pub node: dlist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlist_node {
    pub prev: *mut dlist_node,
    pub next: *mut dlist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelFileNodeBackend {
    pub node: RelFileNode,
    pub backend: BackendId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelFileNode {
    pub spcNode: Oid,
    pub dbNode: Oid,
    pub relNode: Oid,
}
pub type Relation = *mut RelationData;
pub type RelationPtr = *mut Relation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinimalTupleData {
    pub t_len: uint32,
    pub mt_padding: [libc::c_char; 6],
    pub t_infomask2: uint16,
    pub t_infomask: uint16,
    pub t_hoff: uint8,
    pub t_bits: [bits8; 0],
}
pub type MinimalTuple = *mut MinimalTupleData;
pub type HeapTuple = *mut HeapTupleData;
pub type XLogRecPtr = uint64;
pub type TimestampTz = int64;
pub type StringInfo = *mut StringInfoData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pairingheap_node {
    pub first_child: *mut pairingheap_node,
    pub next_sibling: *mut pairingheap_node,
    pub prev_or_parent: *mut pairingheap_node,
}
pub type SnapshotType = libc::c_uint;
pub const SNAPSHOT_NON_VACUUMABLE: SnapshotType = 6;
pub const SNAPSHOT_HISTORIC_MVCC: SnapshotType = 5;
pub const SNAPSHOT_DIRTY: SnapshotType = 4;
pub const SNAPSHOT_TOAST: SnapshotType = 3;
pub const SNAPSHOT_ANY: SnapshotType = 2;
pub const SNAPSHOT_SELF: SnapshotType = 1;
pub const SNAPSHOT_MVCC: SnapshotType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnapshotData {
    pub snapshot_type: SnapshotType,
    pub xmin: TransactionId,
    pub xmax: TransactionId,
    pub xip: *mut TransactionId,
    pub xcnt: uint32,
    pub subxip: *mut TransactionId,
    pub subxcnt: int32,
    pub suboverflowed: bool,
    pub takenDuringRecovery: bool,
    pub copied: bool,
    pub curcid: CommandId,
    pub speculativeToken: uint32,
    pub active_count: uint32,
    pub regd_count: uint32,
    pub ph_node: pairingheap_node,
    pub whenTaken: TimestampTz,
    pub lsn: XLogRecPtr,
}
pub type Snapshot = *mut SnapshotData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexInfo {
    pub type_0: NodeTag,
    pub ii_NumIndexAttrs: libc::c_int,
    pub ii_NumIndexKeyAttrs: libc::c_int,
    pub ii_IndexAttrNumbers: [AttrNumber; 32],
    pub ii_Expressions: *mut List,
    pub ii_ExpressionsState: *mut List,
    pub ii_Predicate: *mut List,
    pub ii_PredicateState: *mut ExprState,
    pub ii_ExclusionOps: *mut Oid,
    pub ii_ExclusionProcs: *mut Oid,
    pub ii_ExclusionStrats: *mut uint16,
    pub ii_UniqueOps: *mut Oid,
    pub ii_UniqueProcs: *mut Oid,
    pub ii_UniqueStrats: *mut uint16,
    pub ii_Unique: bool,
    pub ii_ReadyForInserts: bool,
    pub ii_Concurrent: bool,
    pub ii_BrokenHotChain: bool,
    pub ii_ParallelWorkers: libc::c_int,
    pub ii_Am: Oid,
    pub ii_AmCache: *mut libc::c_void,
    pub ii_Context: MemoryContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprState {
    pub tag: Node,
    pub flags: uint8,
    pub resnull: bool,
    pub resvalue: Datum,
    pub resultslot: *mut TupleTableSlot,
    pub steps: *mut ExprEvalStep,
    pub evalfunc: ExprStateEvalFunc,
    pub expr: *mut Expr,
    pub evalfunc_private: *mut libc::c_void,
    pub steps_len: libc::c_int,
    pub steps_alloc: libc::c_int,
    pub parent: *mut PlanState,
    pub ext_params: ParamListInfo,
    pub innermost_caseval: *mut Datum,
    pub innermost_casenull: *mut bool,
    pub innermost_domainval: *mut Datum,
    pub innermost_domainnull: *mut bool,
}
pub type ParamListInfo = *mut ParamListInfoData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamListInfoData {
    pub paramFetch: ParamFetchHook,
    pub paramFetchArg: *mut libc::c_void,
    pub paramCompile: ParamCompileHook,
    pub paramCompileArg: *mut libc::c_void,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut libc::c_void,
    pub numParams: libc::c_int,
    pub params: [ParamExternData; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExternData {
    pub value: Datum,
    pub isnull: bool,
    pub pflags: uint16,
    pub ptype: Oid,
}
pub type ParserSetupHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseState {
    pub parentParseState: *mut ParseState,
    pub p_sourcetext: *const libc::c_char,
    pub p_rtable: *mut List,
    pub p_joinexprs: *mut List,
    pub p_joinlist: *mut List,
    pub p_namespace: *mut List,
    pub p_lateral_active: bool,
    pub p_ctenamespace: *mut List,
    pub p_future_ctes: *mut List,
    pub p_parent_cte: *mut CommonTableExpr,
    pub p_target_relation: Relation,
    pub p_target_rangetblentry: *mut RangeTblEntry,
    pub p_is_insert: bool,
    pub p_windowdefs: *mut List,
    pub p_expr_kind: ParseExprKind,
    pub p_next_resno: libc::c_int,
    pub p_multiassign_exprs: *mut List,
    pub p_locking_clause: *mut List,
    pub p_locked_from_parent: bool,
    pub p_resolve_unknowns: bool,
    pub p_queryEnv: *mut QueryEnvironment,
    pub p_hasAggs: bool,
    pub p_hasWindowFuncs: bool,
    pub p_hasTargetSRFs: bool,
    pub p_hasSubLinks: bool,
    pub p_hasModifyingCTE: bool,
    pub p_last_srf: *mut Node,
    pub p_pre_columnref_hook: PreParseColumnRefHook,
    pub p_post_columnref_hook: PostParseColumnRefHook,
    pub p_paramref_hook: ParseParamRefHook,
    pub p_coerce_param_hook: CoerceParamHook,
    pub p_ref_hook_state: *mut libc::c_void,
}
pub type CoerceParamHook = Option::<
    unsafe extern "C" fn(
        *mut ParseState,
        *mut Param,
        Oid,
        int32,
        libc::c_int,
    ) -> *mut Node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Param {
    pub xpr: Expr,
    pub paramkind: ParamKind,
    pub paramid: libc::c_int,
    pub paramtype: Oid,
    pub paramtypmod: int32,
    pub paramcollid: Oid,
    pub location: libc::c_int,
}
pub type ParamKind = libc::c_uint;
pub const PARAM_MULTIEXPR: ParamKind = 3;
pub const PARAM_SUBLINK: ParamKind = 2;
pub const PARAM_EXEC: ParamKind = 1;
pub const PARAM_EXTERN: ParamKind = 0;
pub type ParseParamRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamRef {
    pub type_0: NodeTag,
    pub number: libc::c_int,
    pub location: libc::c_int,
}
pub type PostParseColumnRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ColumnRef, *mut Node) -> *mut Node,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColumnRef {
    pub type_0: NodeTag,
    pub fields: *mut List,
    pub location: libc::c_int,
}
pub type PreParseColumnRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ColumnRef) -> *mut Node,
>;
pub type ParseExprKind = libc::c_uint;
pub const EXPR_KIND_GENERATED_COLUMN: ParseExprKind = 40;
pub const EXPR_KIND_COPY_WHERE: ParseExprKind = 39;
pub const EXPR_KIND_CALL_ARGUMENT: ParseExprKind = 38;
pub const EXPR_KIND_PARTITION_EXPRESSION: ParseExprKind = 37;
pub const EXPR_KIND_PARTITION_BOUND: ParseExprKind = 36;
pub const EXPR_KIND_POLICY: ParseExprKind = 35;
pub const EXPR_KIND_TRIGGER_WHEN: ParseExprKind = 34;
pub const EXPR_KIND_EXECUTE_PARAMETER: ParseExprKind = 33;
pub const EXPR_KIND_ALTER_COL_TRANSFORM: ParseExprKind = 32;
pub const EXPR_KIND_INDEX_PREDICATE: ParseExprKind = 31;
pub const EXPR_KIND_INDEX_EXPRESSION: ParseExprKind = 30;
pub const EXPR_KIND_FUNCTION_DEFAULT: ParseExprKind = 29;
pub const EXPR_KIND_COLUMN_DEFAULT: ParseExprKind = 28;
pub const EXPR_KIND_DOMAIN_CHECK: ParseExprKind = 27;
pub const EXPR_KIND_CHECK_CONSTRAINT: ParseExprKind = 26;
pub const EXPR_KIND_VALUES_SINGLE: ParseExprKind = 25;
pub const EXPR_KIND_VALUES: ParseExprKind = 24;
pub const EXPR_KIND_RETURNING: ParseExprKind = 23;
pub const EXPR_KIND_OFFSET: ParseExprKind = 22;
pub const EXPR_KIND_LIMIT: ParseExprKind = 21;
pub const EXPR_KIND_DISTINCT_ON: ParseExprKind = 20;
pub const EXPR_KIND_ORDER_BY: ParseExprKind = 19;
pub const EXPR_KIND_GROUP_BY: ParseExprKind = 18;
pub const EXPR_KIND_UPDATE_TARGET: ParseExprKind = 17;
pub const EXPR_KIND_UPDATE_SOURCE: ParseExprKind = 16;
pub const EXPR_KIND_INSERT_TARGET: ParseExprKind = 15;
pub const EXPR_KIND_SELECT_TARGET: ParseExprKind = 14;
pub const EXPR_KIND_WINDOW_FRAME_GROUPS: ParseExprKind = 13;
pub const EXPR_KIND_WINDOW_FRAME_ROWS: ParseExprKind = 12;
pub const EXPR_KIND_WINDOW_FRAME_RANGE: ParseExprKind = 11;
pub const EXPR_KIND_WINDOW_ORDER: ParseExprKind = 10;
pub const EXPR_KIND_WINDOW_PARTITION: ParseExprKind = 9;
pub const EXPR_KIND_FILTER: ParseExprKind = 8;
pub const EXPR_KIND_HAVING: ParseExprKind = 7;
pub const EXPR_KIND_WHERE: ParseExprKind = 6;
pub const EXPR_KIND_FROM_FUNCTION: ParseExprKind = 5;
pub const EXPR_KIND_FROM_SUBSELECT: ParseExprKind = 4;
pub const EXPR_KIND_JOIN_USING: ParseExprKind = 3;
pub const EXPR_KIND_JOIN_ON: ParseExprKind = 2;
pub const EXPR_KIND_OTHER: ParseExprKind = 1;
pub const EXPR_KIND_NONE: ParseExprKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTblEntry {
    pub type_0: NodeTag,
    pub rtekind: RTEKind,
    pub relid: Oid,
    pub relkind: libc::c_char,
    pub rellockmode: libc::c_int,
    pub tablesample: *mut TableSampleClause,
    pub subquery: *mut Query,
    pub security_barrier: bool,
    pub jointype: JoinType,
    pub joinaliasvars: *mut List,
    pub functions: *mut List,
    pub funcordinality: bool,
    pub tablefunc: *mut TableFunc,
    pub values_lists: *mut List,
    pub ctename: *mut libc::c_char,
    pub ctelevelsup: Index,
    pub self_reference: bool,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub enrname: *mut libc::c_char,
    pub enrtuples: libc::c_double,
    pub alias: *mut Alias,
    pub eref: *mut Alias,
    pub lateral: bool,
    pub inh: bool,
    pub inFromCl: bool,
    pub requiredPerms: AclMode,
    pub checkAsUser: Oid,
    pub selectedCols: *mut Bitmapset,
    pub insertedCols: *mut Bitmapset,
    pub updatedCols: *mut Bitmapset,
    pub extraUpdatedCols: *mut Bitmapset,
    pub securityQuals: *mut List,
}
pub type AclMode = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Alias {
    pub type_0: NodeTag,
    pub aliasname: *mut libc::c_char,
    pub colnames: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableFunc {
    pub type_0: NodeTag,
    pub ns_uris: *mut List,
    pub ns_names: *mut List,
    pub docexpr: *mut Node,
    pub rowexpr: *mut Node,
    pub colnames: *mut List,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub colexprs: *mut List,
    pub coldefexprs: *mut List,
    pub notnulls: *mut Bitmapset,
    pub ordinalitycol: libc::c_int,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Query {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub querySource: QuerySource,
    pub queryId: uint64,
    pub canSetTag: bool,
    pub utilityStmt: *mut Node,
    pub resultRelation: libc::c_int,
    pub hasAggs: bool,
    pub hasWindowFuncs: bool,
    pub hasTargetSRFs: bool,
    pub hasSubLinks: bool,
    pub hasDistinctOn: bool,
    pub hasRecursive: bool,
    pub hasModifyingCTE: bool,
    pub hasForUpdate: bool,
    pub hasRowSecurity: bool,
    pub cteList: *mut List,
    pub rtable: *mut List,
    pub jointree: *mut FromExpr,
    pub targetList: *mut List,
    pub override_0: OverridingKind,
    pub onConflict: *mut OnConflictExpr,
    pub returningList: *mut List,
    pub groupClause: *mut List,
    pub groupingSets: *mut List,
    pub havingQual: *mut Node,
    pub windowClause: *mut List,
    pub distinctClause: *mut List,
    pub sortClause: *mut List,
    pub limitOffset: *mut Node,
    pub limitCount: *mut Node,
    pub rowMarks: *mut List,
    pub setOperations: *mut Node,
    pub constraintDeps: *mut List,
    pub withCheckOptions: *mut List,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictExpr {
    pub type_0: NodeTag,
    pub action: OnConflictAction,
    pub arbiterElems: *mut List,
    pub arbiterWhere: *mut Node,
    pub constraint: Oid,
    pub onConflictSet: *mut List,
    pub onConflictWhere: *mut Node,
    pub exclRelIndex: libc::c_int,
    pub exclRelTlist: *mut List,
}
pub type OverridingKind = libc::c_uint;
pub const OVERRIDING_SYSTEM_VALUE: OverridingKind = 2;
pub const OVERRIDING_USER_VALUE: OverridingKind = 1;
pub const OVERRIDING_NOT_SET: OverridingKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FromExpr {
    pub type_0: NodeTag,
    pub fromlist: *mut List,
    pub quals: *mut Node,
}
pub type QuerySource = libc::c_uint;
pub const QSRC_NON_INSTEAD_RULE: QuerySource = 4;
pub const QSRC_QUAL_INSTEAD_RULE: QuerySource = 3;
pub const QSRC_INSTEAD_RULE: QuerySource = 2;
pub const QSRC_PARSER: QuerySource = 1;
pub const QSRC_ORIGINAL: QuerySource = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableSampleClause {
    pub type_0: NodeTag,
    pub tsmhandler: Oid,
    pub args: *mut List,
    pub repeatable: *mut Expr,
}
pub type RTEKind = libc::c_uint;
pub const RTE_RESULT: RTEKind = 8;
pub const RTE_NAMEDTUPLESTORE: RTEKind = 7;
pub const RTE_CTE: RTEKind = 6;
pub const RTE_VALUES: RTEKind = 5;
pub const RTE_TABLEFUNC: RTEKind = 4;
pub const RTE_FUNCTION: RTEKind = 3;
pub const RTE_JOIN: RTEKind = 2;
pub const RTE_SUBQUERY: RTEKind = 1;
pub const RTE_RELATION: RTEKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonTableExpr {
    pub type_0: NodeTag,
    pub ctename: *mut libc::c_char,
    pub aliascolnames: *mut List,
    pub ctematerialized: CTEMaterialize,
    pub ctequery: *mut Node,
    pub location: libc::c_int,
    pub cterecursive: bool,
    pub cterefcount: libc::c_int,
    pub ctecolnames: *mut List,
    pub ctecoltypes: *mut List,
    pub ctecoltypmods: *mut List,
    pub ctecolcollations: *mut List,
}
pub type CTEMaterialize = libc::c_uint;
pub const CTEMaterializeNever: CTEMaterialize = 2;
pub const CTEMaterializeAlways: CTEMaterialize = 1;
pub const CTEMaterializeDefault: CTEMaterialize = 0;
pub type ParamCompileHook = Option::<
    unsafe extern "C" fn(
        ParamListInfo,
        *mut Param,
        *mut ExprState,
        *mut Datum,
        *mut bool,
    ) -> (),
>;
pub type ParamFetchHook = Option::<
    unsafe extern "C" fn(
        ParamListInfo,
        libc::c_int,
        bool,
        *mut ParamExternData,
    ) -> *mut ParamExternData,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanState {
    pub type_0: NodeTag,
    pub plan: *mut Plan,
    pub state: *mut EState,
    pub ExecProcNode: ExecProcNodeMtd,
    pub ExecProcNodeReal: ExecProcNodeMtd,
    pub instrument: *mut Instrumentation,
    pub worker_instrument: *mut WorkerInstrumentation,
    pub worker_jit_instrument: *mut SharedJitInstrumentation,
    pub qual: *mut ExprState,
    pub lefttree: *mut PlanState,
    pub righttree: *mut PlanState,
    pub initPlan: *mut List,
    pub subPlan: *mut List,
    pub chgParam: *mut Bitmapset,
    pub ps_ResultTupleDesc: TupleDesc,
    pub ps_ResultTupleSlot: *mut TupleTableSlot,
    pub ps_ExprContext: *mut ExprContext,
    pub ps_ProjInfo: *mut ProjectionInfo,
    pub scandesc: TupleDesc,
    pub scanops: *const TupleTableSlotOps,
    pub outerops: *const TupleTableSlotOps,
    pub innerops: *const TupleTableSlotOps,
    pub resultops: *const TupleTableSlotOps,
    pub scanopsfixed: bool,
    pub outeropsfixed: bool,
    pub inneropsfixed: bool,
    pub resultopsfixed: bool,
    pub scanopsset: bool,
    pub outeropsset: bool,
    pub inneropsset: bool,
    pub resultopsset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlotOps {
    pub base_slot_size: size_t,
    pub init: Option::<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub release: Option::<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub clear: Option::<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub getsomeattrs: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int) -> (),
    >,
    pub getsysattr: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int, *mut bool) -> Datum,
    >,
    pub materialize: Option::<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub copyslot: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot, *mut TupleTableSlot) -> (),
    >,
    pub get_heap_tuple: Option::<unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple>,
    pub get_minimal_tuple: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple,
    >,
    pub copy_heap_tuple: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple,
    >,
    pub copy_minimal_tuple: Option::<
        unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlot {
    pub type_0: NodeTag,
    pub tts_flags: uint16,
    pub tts_nvalid: AttrNumber,
    pub tts_ops: *const TupleTableSlotOps,
    pub tts_tupleDescriptor: TupleDesc,
    pub tts_values: *mut Datum,
    pub tts_isnull: *mut bool,
    pub tts_mcxt: MemoryContext,
    pub tts_tid: ItemPointerData,
    pub tts_tableOid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProjectionInfo {
    pub type_0: NodeTag,
    pub pi_state: ExprState,
    pub pi_exprContext: *mut ExprContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext {
    pub type_0: NodeTag,
    pub ecxt_scantuple: *mut TupleTableSlot,
    pub ecxt_innertuple: *mut TupleTableSlot,
    pub ecxt_outertuple: *mut TupleTableSlot,
    pub ecxt_per_query_memory: MemoryContext,
    pub ecxt_per_tuple_memory: MemoryContext,
    pub ecxt_param_exec_vals: *mut ParamExecData,
    pub ecxt_param_list_info: ParamListInfo,
    pub ecxt_aggvalues: *mut Datum,
    pub ecxt_aggnulls: *mut bool,
    pub caseValue_datum: Datum,
    pub caseValue_isNull: bool,
    pub domainValue_datum: Datum,
    pub domainValue_isNull: bool,
    pub ecxt_estate: *mut EState,
    pub ecxt_callbacks: *mut ExprContext_CB,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext_CB {
    pub next: *mut ExprContext_CB,
    pub function: ExprContextCallbackFunction,
    pub arg: Datum,
}
pub type ExprContextCallbackFunction = Option::<unsafe extern "C" fn(Datum) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub type_0: NodeTag,
    pub es_direction: ScanDirection,
    pub es_snapshot: Snapshot,
    pub es_crosscheck_snapshot: Snapshot,
    pub es_range_table: *mut List,
    pub es_range_table_array: *mut *mut RangeTblEntry,
    pub es_range_table_size: Index,
    pub es_relations: *mut Relation,
    pub es_rowmarks: *mut *mut ExecRowMark,
    pub es_plannedstmt: *mut PlannedStmt,
    pub es_sourceText: *const libc::c_char,
    pub es_junkFilter: *mut JunkFilter,
    pub es_output_cid: CommandId,
    pub es_result_relations: *mut ResultRelInfo,
    pub es_num_result_relations: libc::c_int,
    pub es_result_relation_info: *mut ResultRelInfo,
    pub es_root_result_relations: *mut ResultRelInfo,
    pub es_num_root_result_relations: libc::c_int,
    pub es_partition_directory: PartitionDirectory,
    pub es_tuple_routing_result_relations: *mut List,
    pub es_trig_target_relations: *mut List,
    pub es_param_list_info: ParamListInfo,
    pub es_param_exec_vals: *mut ParamExecData,
    pub es_queryEnv: *mut QueryEnvironment,
    pub es_query_cxt: MemoryContext,
    pub es_tupleTable: *mut List,
    pub es_processed: uint64,
    pub es_top_eflags: libc::c_int,
    pub es_instrument: libc::c_int,
    pub es_finished: bool,
    pub es_exprcontexts: *mut List,
    pub es_subplanstates: *mut List,
    pub es_auxmodifytables: *mut List,
    pub es_per_tuple_exprcontext: *mut ExprContext,
    pub es_epq_active: *mut EPQState,
    pub es_use_parallel_mode: bool,
    pub es_query_dsa: *mut dsa_area,
    pub es_jit_flags: libc::c_int,
    pub es_jit: *mut JitContext,
    pub es_jit_worker_instr: *mut JitInstrumentation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EPQState {
    pub parentestate: *mut EState,
    pub epqParam: libc::c_int,
    pub tuple_table: *mut List,
    pub relsubs_slot: *mut *mut TupleTableSlot,
    pub plan: *mut Plan,
    pub arowMarks: *mut List,
    pub origslot: *mut TupleTableSlot,
    pub recheckestate: *mut EState,
    pub relsubs_rowmark: *mut *mut ExecAuxRowMark,
    pub relsubs_done: *mut bool,
    pub recheckplanstate: *mut PlanState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecAuxRowMark {
    pub rowmark: *mut ExecRowMark,
    pub ctidAttNo: AttrNumber,
    pub toidAttNo: AttrNumber,
    pub wholeAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecRowMark {
    pub relation: Relation,
    pub relid: Oid,
    pub rti: Index,
    pub prti: Index,
    pub rowmarkId: Index,
    pub markType: RowMarkType,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
    pub ermActive: bool,
    pub curCtid: ItemPointerData,
    pub ermExtra: *mut libc::c_void,
}
pub type LockWaitPolicy = libc::c_uint;
pub const LockWaitError: LockWaitPolicy = 2;
pub const LockWaitSkip: LockWaitPolicy = 1;
pub const LockWaitBlock: LockWaitPolicy = 0;
pub type LockClauseStrength = libc::c_uint;
pub const LCS_FORUPDATE: LockClauseStrength = 4;
pub const LCS_FORNOKEYUPDATE: LockClauseStrength = 3;
pub const LCS_FORSHARE: LockClauseStrength = 2;
pub const LCS_FORKEYSHARE: LockClauseStrength = 1;
pub const LCS_NONE: LockClauseStrength = 0;
pub type RowMarkType = libc::c_uint;
pub const ROW_MARK_COPY: RowMarkType = 5;
pub const ROW_MARK_REFERENCE: RowMarkType = 4;
pub const ROW_MARK_KEYSHARE: RowMarkType = 3;
pub const ROW_MARK_SHARE: RowMarkType = 2;
pub const ROW_MARK_NOKEYEXCLUSIVE: RowMarkType = 1;
pub const ROW_MARK_EXCLUSIVE: RowMarkType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plan {
    pub type_0: NodeTag,
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub plan_rows: libc::c_double,
    pub plan_width: libc::c_int,
    pub parallel_aware: bool,
    pub parallel_safe: bool,
    pub plan_node_id: libc::c_int,
    pub targetlist: *mut List,
    pub qual: *mut List,
    pub lefttree: *mut Plan,
    pub righttree: *mut Plan,
    pub initPlan: *mut List,
    pub extParam: *mut Bitmapset,
    pub allParam: *mut Bitmapset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExecData {
    pub execPlan: *mut libc::c_void,
    pub value: Datum,
    pub isnull: bool,
}
pub type PartitionDirectory = *mut PartitionDirectoryData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResultRelInfo {
    pub type_0: NodeTag,
    pub ri_RangeTableIndex: Index,
    pub ri_RelationDesc: Relation,
    pub ri_NumIndices: libc::c_int,
    pub ri_IndexRelationDescs: RelationPtr,
    pub ri_IndexRelationInfo: *mut *mut IndexInfo,
    pub ri_TrigDesc: *mut TriggerDesc,
    pub ri_TrigFunctions: *mut FmgrInfo,
    pub ri_TrigWhenExprs: *mut *mut ExprState,
    pub ri_TrigInstrument: *mut Instrumentation,
    pub ri_ReturningSlot: *mut TupleTableSlot,
    pub ri_TrigOldSlot: *mut TupleTableSlot,
    pub ri_TrigNewSlot: *mut TupleTableSlot,
    pub ri_FdwRoutine: *mut FdwRoutine,
    pub ri_FdwState: *mut libc::c_void,
    pub ri_usesFdwDirectModify: bool,
    pub ri_WithCheckOptions: *mut List,
    pub ri_WithCheckOptionExprs: *mut List,
    pub ri_ConstraintExprs: *mut *mut ExprState,
    pub ri_GeneratedExprs: *mut *mut ExprState,
    pub ri_junkFilter: *mut JunkFilter,
    pub ri_returningList: *mut List,
    pub ri_projectReturning: *mut ProjectionInfo,
    pub ri_onConflictArbiterIndexes: *mut List,
    pub ri_onConflict: *mut OnConflictSetState,
    pub ri_PartitionCheck: *mut List,
    pub ri_PartitionCheckExpr: *mut ExprState,
    pub ri_RootResultRelInfo: *mut ResultRelInfo,
    pub ri_PartitionInfo: *mut PartitionRoutingInfo,
    pub ri_CopyMultiInsertBuffer: *mut CopyMultiInsertBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictSetState {
    pub type_0: NodeTag,
    pub oc_Existing: *mut TupleTableSlot,
    pub oc_ProjSlot: *mut TupleTableSlot,
    pub oc_ProjInfo: *mut ProjectionInfo,
    pub oc_WhereClause: *mut ExprState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JunkFilter {
    pub type_0: NodeTag,
    pub jf_targetList: *mut List,
    pub jf_cleanTupType: TupleDesc,
    pub jf_cleanMap: *mut AttrNumber,
    pub jf_resultSlot: *mut TupleTableSlot,
    pub jf_junkAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrumentation {
    pub need_timer: bool,
    pub need_bufusage: bool,
    pub running: bool,
    pub starttime: instr_time,
    pub counter: instr_time,
    pub firsttuple: libc::c_double,
    pub tuplecount: libc::c_double,
    pub bufusage_start: BufferUsage,
    pub startup: libc::c_double,
    pub total: libc::c_double,
    pub ntuples: libc::c_double,
    pub ntuples2: libc::c_double,
    pub nloops: libc::c_double,
    pub nfiltered1: libc::c_double,
    pub nfiltered2: libc::c_double,
    pub bufusage: BufferUsage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferUsage {
    pub shared_blks_hit: libc::c_long,
    pub shared_blks_read: libc::c_long,
    pub shared_blks_dirtied: libc::c_long,
    pub shared_blks_written: libc::c_long,
    pub local_blks_hit: libc::c_long,
    pub local_blks_read: libc::c_long,
    pub local_blks_dirtied: libc::c_long,
    pub local_blks_written: libc::c_long,
    pub temp_blks_read: libc::c_long,
    pub temp_blks_written: libc::c_long,
    pub blk_read_time: instr_time,
    pub blk_write_time: instr_time,
}
pub type instr_time = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlannedStmt {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub queryId: uint64,
    pub hasReturning: bool,
    pub hasModifyingCTE: bool,
    pub canSetTag: bool,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeNeeded: bool,
    pub jitFlags: libc::c_int,
    pub planTree: *mut Plan,
    pub rtable: *mut List,
    pub resultRelations: *mut List,
    pub rootResultRelations: *mut List,
    pub subplans: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub utilityStmt: *mut Node,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkerInstrumentation {
    pub num_workers: libc::c_int,
    pub instrument: [Instrumentation; 0],
}
pub type ExecProcNodeMtd = Option::<
    unsafe extern "C" fn(*mut PlanState) -> *mut TupleTableSlot,
>;
pub type ExprStateEvalFunc = Option::<
    unsafe extern "C" fn(*mut ExprState, *mut ExprContext, *mut bool) -> Datum,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayType {
    pub vl_len_: int32,
    pub ndim: libc::c_int,
    pub dataoffset: int32,
    pub elemtype: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectAddress {
    pub classId: Oid,
    pub objectId: Oid,
    pub objectSubId: int32,
}
pub type DependencyType = libc::c_uint;
pub const DEPENDENCY_PIN: DependencyType = 112;
pub const DEPENDENCY_AUTO_EXTENSION: DependencyType = 120;
pub const DEPENDENCY_EXTENSION: DependencyType = 101;
pub const DEPENDENCY_PARTITION_SEC: DependencyType = 83;
pub const DEPENDENCY_PARTITION_PRI: DependencyType = 80;
pub const DEPENDENCY_INTERNAL: DependencyType = 105;
pub const DEPENDENCY_AUTO: DependencyType = 97;
pub const DEPENDENCY_NORMAL: DependencyType = 110;
pub type CatalogIndexState = *mut ResultRelInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_opclass {
    pub oid: Oid,
    pub opcmethod: Oid,
    pub opcname: NameData,
    pub opcnamespace: Oid,
    pub opcowner: Oid,
    pub opcfamily: Oid,
    pub opcintype: Oid,
    pub opcdefault: bool,
    pub opckeytype: Oid,
}
pub type Form_pg_opclass = *mut FormData_pg_opclass;
pub type TriggerEvent = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerData {
    pub type_0: NodeTag,
    pub tg_event: TriggerEvent,
    pub tg_relation: Relation,
    pub tg_trigtuple: HeapTuple,
    pub tg_newtuple: HeapTuple,
    pub tg_trigger: *mut Trigger,
    pub tg_trigslot: *mut TupleTableSlot,
    pub tg_newslot: *mut TupleTableSlot,
    pub tg_oldtable: *mut Tuplestorestate,
    pub tg_newtable: *mut Tuplestorestate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slist_node {
    pub next: *mut slist_node,
}
pub type SysCacheIdentifier = libc::c_uint;
pub const USERMAPPINGUSERSERVER: SysCacheIdentifier = 77;
pub const USERMAPPINGOID: SysCacheIdentifier = 76;
pub const TYPEOID: SysCacheIdentifier = 75;
pub const TYPENAMENSP: SysCacheIdentifier = 74;
pub const TSTEMPLATEOID: SysCacheIdentifier = 73;
pub const TSTEMPLATENAMENSP: SysCacheIdentifier = 72;
pub const TSPARSEROID: SysCacheIdentifier = 71;
pub const TSPARSERNAMENSP: SysCacheIdentifier = 70;
pub const TSDICTOID: SysCacheIdentifier = 69;
pub const TSDICTNAMENSP: SysCacheIdentifier = 68;
pub const TSCONFIGOID: SysCacheIdentifier = 67;
pub const TSCONFIGNAMENSP: SysCacheIdentifier = 66;
pub const TSCONFIGMAP: SysCacheIdentifier = 65;
pub const TRFTYPELANG: SysCacheIdentifier = 64;
pub const TRFOID: SysCacheIdentifier = 63;
pub const TABLESPACEOID: SysCacheIdentifier = 62;
pub const SUBSCRIPTIONRELMAP: SysCacheIdentifier = 61;
pub const SUBSCRIPTIONOID: SysCacheIdentifier = 60;
pub const SUBSCRIPTIONNAME: SysCacheIdentifier = 59;
pub const STATRELATTINH: SysCacheIdentifier = 58;
pub const STATEXTOID: SysCacheIdentifier = 57;
pub const STATEXTNAMENSP: SysCacheIdentifier = 56;
pub const STATEXTDATASTXOID: SysCacheIdentifier = 55;
pub const SEQRELID: SysCacheIdentifier = 54;
pub const RULERELNAME: SysCacheIdentifier = 53;
pub const REPLORIGNAME: SysCacheIdentifier = 52;
pub const REPLORIGIDENT: SysCacheIdentifier = 51;
pub const RELOID: SysCacheIdentifier = 50;
pub const RELNAMENSP: SysCacheIdentifier = 49;
pub const RANGETYPE: SysCacheIdentifier = 48;
pub const PUBLICATIONRELMAP: SysCacheIdentifier = 47;
pub const PUBLICATIONREL: SysCacheIdentifier = 46;
pub const PUBLICATIONOID: SysCacheIdentifier = 45;
pub const PUBLICATIONNAME: SysCacheIdentifier = 44;
pub const PROCOID: SysCacheIdentifier = 43;
pub const PROCNAMEARGSNSP: SysCacheIdentifier = 42;
pub const PARTRELID: SysCacheIdentifier = 41;
pub const OPFAMILYOID: SysCacheIdentifier = 40;
pub const OPFAMILYAMNAMENSP: SysCacheIdentifier = 39;
pub const OPEROID: SysCacheIdentifier = 38;
pub const OPERNAMENSP: SysCacheIdentifier = 37;
pub const NAMESPACEOID: SysCacheIdentifier = 36;
pub const NAMESPACENAME: SysCacheIdentifier = 35;
pub const LANGOID: SysCacheIdentifier = 34;
pub const LANGNAME: SysCacheIdentifier = 33;
pub const INDEXRELID: SysCacheIdentifier = 32;
pub const FOREIGNTABLEREL: SysCacheIdentifier = 31;
pub const FOREIGNSERVEROID: SysCacheIdentifier = 30;
pub const FOREIGNSERVERNAME: SysCacheIdentifier = 29;
pub const FOREIGNDATAWRAPPEROID: SysCacheIdentifier = 28;
pub const FOREIGNDATAWRAPPERNAME: SysCacheIdentifier = 27;
pub const EVENTTRIGGEROID: SysCacheIdentifier = 26;
pub const EVENTTRIGGERNAME: SysCacheIdentifier = 25;
pub const ENUMTYPOIDNAME: SysCacheIdentifier = 24;
pub const ENUMOID: SysCacheIdentifier = 23;
pub const DEFACLROLENSPOBJ: SysCacheIdentifier = 22;
pub const DATABASEOID: SysCacheIdentifier = 21;
pub const CONVOID: SysCacheIdentifier = 20;
pub const CONSTROID: SysCacheIdentifier = 19;
pub const CONNAMENSP: SysCacheIdentifier = 18;
pub const CONDEFAULT: SysCacheIdentifier = 17;
pub const COLLOID: SysCacheIdentifier = 16;
pub const COLLNAMEENCNSP: SysCacheIdentifier = 15;
pub const CLAOID: SysCacheIdentifier = 14;
pub const CLAAMNAMENSP: SysCacheIdentifier = 13;
pub const CASTSOURCETARGET: SysCacheIdentifier = 12;
pub const AUTHOID: SysCacheIdentifier = 11;
pub const AUTHNAME: SysCacheIdentifier = 10;
pub const AUTHMEMROLEMEM: SysCacheIdentifier = 9;
pub const AUTHMEMMEMROLE: SysCacheIdentifier = 8;
pub const ATTNUM: SysCacheIdentifier = 7;
pub const ATTNAME: SysCacheIdentifier = 6;
pub const AMPROCNUM: SysCacheIdentifier = 5;
pub const AMOPSTRATEGY: SysCacheIdentifier = 4;
pub const AMOPOPID: SysCacheIdentifier = 3;
pub const AMOID: SysCacheIdentifier = 2;
pub const AMNAME: SysCacheIdentifier = 1;
pub const AGGFNOID: SysCacheIdentifier = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPITupleTable {
    pub tuptabcxt: MemoryContext,
    pub alloced: uint64,
    pub free: uint64,
    pub tupdesc: TupleDesc,
    pub vals: *mut HeapTuple,
    pub next: slist_node,
    pub subid: SubTransactionId,
}
pub type SPIPlanPtr = *mut _SPI_plan;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexDef {
    pub create: *mut libc::c_char,
    pub index: *mut libc::c_char,
    pub table: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub columns: *mut libc::c_char,
    pub options: *mut libc::c_char,
    pub tablespace: *mut libc::c_char,
    pub where_0: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_head(mut l: *const List) -> *mut ListCell {
    return if !l.is_null() { (*l).head } else { 0 as *mut ListCell };
}
#[inline]
unsafe extern "C" fn list_length(mut l: *const List) -> libc::c_int {
    return if !l.is_null() { (*l).length } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn Pg_magic_func() -> *const Pg_magic_struct {
    static mut Pg_magic_data: Pg_magic_struct = {
        let mut init = Pg_magic_struct {
            len: ::std::mem::size_of::<Pg_magic_struct>() as libc::c_ulong
                as libc::c_int,
            version: 120022 as libc::c_int / 100 as libc::c_int,
            funcmaxargs: 100 as libc::c_int,
            indexmaxkeys: 32 as libc::c_int,
            namedatalen: 64 as libc::c_int,
            float4byval: 1 as libc::c_int,
            float8byval: 1 as libc::c_int,
        };
        init
    };
    return &Pg_magic_data;
}
pub unsafe extern "C" fn pg_finfo_repack_version() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_trigger() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_apply() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_get_order_by() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_indexdef() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_swap() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_drop() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_disable_autovacuum() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_index_swap() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
pub unsafe extern "C" fn pg_finfo_repack_get_table_and_inheritors() -> *const Pg_finfo_record {
    static mut my_finfo: Pg_finfo_record = {
        let mut init = Pg_finfo_record {
            api_version: 1 as libc::c_int,
        };
        init
    };
    return &my_finfo;
}
unsafe extern "C" fn must_be_superuser(mut func: *const libc::c_char) {
    if !superuser() {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"must_be_superuser\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"must be superuser to use %s function\0" as *const u8
                as *const libc::c_char,
            func,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
}
pub unsafe extern "C" fn repack_version(mut fcinfo: FunctionCallInfo) -> Datum {
    return cstring_to_text(b"pg_repack 1.4.7\0" as *const u8 as *const libc::c_char)
        as Datum;
}
pub unsafe extern "C" fn repack_trigger(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut trigdata: *mut TriggerData = (*fcinfo).context as *mut TriggerData;
    let mut desc: TupleDesc = 0 as *mut TupleDescData;
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    let mut values: [Datum; 2] = [0; 2];
    let mut nulls: [bool; 2] = [0 as libc::c_int != 0, 0 as libc::c_int != 0];
    let mut argtypes: [Oid; 2] = [0; 2];
    let mut sql: *const libc::c_char = 0 as *const libc::c_char;
    must_be_superuser(b"repack_trigger\0" as *const u8 as *const libc::c_char);
    if !(!((*fcinfo).context).is_null()
        && (*((*fcinfo).context as *const Node)).type_0 as libc::c_uint
            == T_TriggerData as libc::c_int as libc::c_uint)
        || !((*trigdata).tg_event & 0x18 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        || (*trigdata).tg_event & 0x4 as libc::c_int as libc::c_uint == 0
        || (*(*trigdata).tg_trigger).tgnargs as libc::c_int != 1 as libc::c_int
    {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"repack_trigger\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"repack_trigger: invalid trigger call\0" as *const u8 as *const libc::c_char,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    sql = *((*(*trigdata).tg_trigger).tgargs).offset(0 as libc::c_int as isize);
    desc = (*(*trigdata).tg_relation).rd_att;
    argtypes[1 as libc::c_int as usize] = (*(*(*trigdata).tg_relation).rd_rel).reltype;
    argtypes[0 as libc::c_int as usize] = argtypes[1 as libc::c_int as usize];
    repack_init();
    if (*trigdata).tg_event & 0x3 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        tuple = (*trigdata).tg_trigtuple;
        nulls[0 as libc::c_int as usize] = 1 as libc::c_int != 0;
        values[1 as libc::c_int as usize] = SPI_returntuple(tuple, desc) as Datum;
    } else if (*trigdata).tg_event & 0x3 as libc::c_int as libc::c_uint
        == 0x1 as libc::c_int as libc::c_uint
    {
        tuple = (*trigdata).tg_trigtuple;
        values[0 as libc::c_int as usize] = SPI_returntuple(tuple, desc) as Datum;
        nulls[1 as libc::c_int as usize] = 1 as libc::c_int != 0;
    } else {
        tuple = (*trigdata).tg_newtuple;
        values[0 as libc::c_int
            as usize] = SPI_returntuple((*trigdata).tg_trigtuple, desc) as Datum;
        values[1 as libc::c_int as usize] = SPI_returntuple(tuple, desc) as Datum;
    }
    execute_with_args(
        7 as libc::c_int,
        sql,
        2 as libc::c_int,
        argtypes.as_mut_ptr(),
        values.as_mut_ptr(),
        nulls.as_mut_ptr() as *const bool,
    );
    SPI_finish();
    return tuple as Datum;
}
pub unsafe extern "C" fn repack_apply(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut sql_peek: *const libc::c_char = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .value as Pointer;
    let mut sql_insert: *const libc::c_char = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
        .value as Pointer;
    let mut sql_delete: *const libc::c_char = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize))
        .value as Pointer;
    let mut sql_update: *const libc::c_char = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize))
        .value as Pointer;
    let mut count: int32 = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(5 as libc::c_int as isize))
        .value as int32;
    let mut plan_peek: SPIPlanPtr = 0 as SPIPlanPtr;
    let mut plan_insert: SPIPlanPtr = 0 as SPIPlanPtr;
    let mut plan_delete: SPIPlanPtr = 0 as SPIPlanPtr;
    let mut plan_update: SPIPlanPtr = 0 as SPIPlanPtr;
    let mut n: uint32 = 0;
    let mut i: uint32 = 0;
    let mut argtypes_peek: [Oid; 1] = [23 as libc::c_int as Oid];
    let mut values_peek: [Datum; 1] = [0; 1];
    let nulls_peek: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
    let mut sql_pop: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    initStringInfo(&mut sql_pop);
    must_be_superuser(b"repack_apply\0" as *const u8 as *const libc::c_char);
    repack_init();
    plan_peek = repack_prepare(sql_peek, 1 as libc::c_int, argtypes_peek.as_mut_ptr());
    n = 0 as libc::c_int as uint32;
    loop {
        let mut ntuples: libc::c_int = 0;
        let mut tuptable: *mut SPITupleTable = 0 as *mut SPITupleTable;
        let mut desc: TupleDesc = 0 as *mut TupleDescData;
        let mut argtypes: [Oid; 3] = [0; 3];
        let mut values: [Datum; 3] = [0; 3];
        let mut nulls: [bool; 3] = [false; 3];
        if count <= 0 as libc::c_int {
            values_peek[0 as libc::c_int as usize] = 1000 as libc::c_int as Datum;
        } else {
            values_peek[0 as libc::c_int
                as usize] = (if (count as libc::c_uint).wrapping_sub(n)
                < 1000 as libc::c_int as libc::c_uint
            {
                (count as libc::c_uint).wrapping_sub(n)
            } else {
                1000 as libc::c_int as libc::c_uint
            }) as Datum;
        }
        execute_plan(
            5 as libc::c_int,
            plan_peek,
            values_peek.as_mut_ptr(),
            nulls_peek.as_ptr(),
        );
        if SPI_processed <= 0 as libc::c_int as libc::c_ulong {
            break;
        }
        ntuples = SPI_processed as libc::c_int;
        tuptable = SPI_tuptable;
        desc = (*tuptable).tupdesc;
        argtypes[0 as libc::c_int as usize] = SPI_gettypeid(desc, 1 as libc::c_int);
        argtypes[1 as libc::c_int as usize] = SPI_gettypeid(desc, 2 as libc::c_int);
        argtypes[2 as libc::c_int as usize] = SPI_gettypeid(desc, 3 as libc::c_int);
        resetStringInfo(&mut sql_pop);
        appendStringInfoString(
            &mut sql_pop,
            (*((*fcinfo).args).as_mut_ptr().offset(4 as libc::c_int as isize)).value
                as Pointer,
        );
        i = 0 as libc::c_int as uint32;
        while i < ntuples as libc::c_uint {
            let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
            let mut pkid: *mut libc::c_char = 0 as *mut libc::c_char;
            tuple = *((*tuptable).vals).offset(i as isize);
            values[0 as libc::c_int
                as usize] = SPI_getbinval(
                tuple,
                desc,
                1 as libc::c_int,
                &mut *nulls.as_mut_ptr().offset(0 as libc::c_int as isize),
            );
            values[1 as libc::c_int
                as usize] = SPI_getbinval(
                tuple,
                desc,
                2 as libc::c_int,
                &mut *nulls.as_mut_ptr().offset(1 as libc::c_int as isize),
            );
            values[2 as libc::c_int
                as usize] = SPI_getbinval(
                tuple,
                desc,
                3 as libc::c_int,
                &mut *nulls.as_mut_ptr().offset(2 as libc::c_int as isize),
            );
            pkid = SPI_getvalue(tuple, desc, 1 as libc::c_int);
            if nulls[1 as libc::c_int as usize] {
                if plan_insert.is_null() {
                    plan_insert = repack_prepare(
                        sql_insert,
                        1 as libc::c_int,
                        &mut *argtypes.as_mut_ptr().offset(2 as libc::c_int as isize),
                    );
                }
                execute_plan(
                    7 as libc::c_int,
                    plan_insert,
                    &mut *values.as_mut_ptr().offset(2 as libc::c_int as isize),
                    if nulls[2 as libc::c_int as usize] as libc::c_int != 0 {
                        b"n\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    },
                );
            } else if nulls[2 as libc::c_int as usize] {
                if plan_delete.is_null() {
                    plan_delete = repack_prepare(
                        sql_delete,
                        1 as libc::c_int,
                        &mut *argtypes.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                }
                execute_plan(
                    8 as libc::c_int,
                    plan_delete,
                    &mut *values.as_mut_ptr().offset(1 as libc::c_int as isize),
                    if nulls[1 as libc::c_int as usize] as libc::c_int != 0 {
                        b"n\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    },
                );
            } else {
                if plan_update.is_null() {
                    plan_update = repack_prepare(
                        sql_update,
                        2 as libc::c_int,
                        &mut *argtypes.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                }
                execute_plan(
                    9 as libc::c_int,
                    plan_update,
                    &mut *values.as_mut_ptr().offset(1 as libc::c_int as isize),
                    if nulls[1 as libc::c_int as usize] as libc::c_int != 0 {
                        b"n\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    },
                );
            }
            if i == 0 as libc::c_int as libc::c_uint {
                appendStringInfoString(&mut sql_pop, pkid);
            } else {
                appendStringInfo(
                    &mut sql_pop as *mut StringInfoData,
                    b",%s\0" as *const u8 as *const libc::c_char,
                    pkid,
                );
            }
            pfree(pkid as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
            n = n.wrapping_add(1);
            n;
        }
        appendStringInfoString(
            &mut sql_pop,
            b");\0" as *const u8 as *const libc::c_char,
        );
        execute(8 as libc::c_int, sql_pop.data);
        SPI_freetuptable(tuptable);
    }
    SPI_finish();
    return n as Datum;
}
unsafe extern "C" fn get_relation_name(mut relid: Oid) -> *mut libc::c_char {
    let mut nsp: Oid = get_rel_namespace(relid);
    let mut nspname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strver: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ver: libc::c_int = 0;
    strver = GetConfigOptionByName(
        b"server_version_num\0" as *const u8 as *const libc::c_char,
        0 as *mut *const libc::c_char,
        0 as libc::c_int != 0,
    );
    ver = atoi(strver);
    pfree(strver as *mut libc::c_void);
    if ver >= 100000 as libc::c_int && ver < 100003 as libc::c_int
        || ver >= 90600 as libc::c_int && ver < 90608 as libc::c_int
        || ver >= 90500 as libc::c_int && ver < 90512 as libc::c_int
        || ver >= 90400 as libc::c_int && ver < 90417 as libc::c_int
        || ver >= 90300 as libc::c_int && ver < 90322 as libc::c_int
        || ver >= 90200 as libc::c_int && ver < 90300 as libc::c_int
        || ver >= 90100 as libc::c_int && ver < 90200 as libc::c_int
    {
        if RelationIsVisible(relid) {
            nspname = 0 as *mut libc::c_char;
        } else {
            nspname = get_namespace_name(nsp);
        }
    } else if nsp != 0 as libc::c_int as Oid {
        nspname = get_namespace_name(nsp);
    } else {
        nspname = 0 as *mut libc::c_char;
    }
    return quote_qualified_identifier(nspname, get_rel_name(relid));
}
unsafe extern "C" fn parse_error(mut index: Oid) -> *mut libc::c_char {
    let mut __errno_location: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        421 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"parse_error\0"))
            .as_ptr(),
    );
    elog_finish(
        20 as libc::c_int,
        b"unexpected index definition: %s\0" as *const u8 as *const libc::c_char,
        pg_get_indexdef_string(index),
    );
    if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn skip_const(
    mut index: Oid,
    mut sql: *mut libc::c_char,
    mut arg1: *const libc::c_char,
    mut arg2: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    if !arg1.is_null()
        && {
            len = strlen(arg1);
            strncmp(sql, arg1, len) == 0 as libc::c_int
        }
        || !arg2.is_null()
            && {
                len = strlen(arg2);
                strncmp(sql, arg2, len) == 0 as libc::c_int
            }
    {
        *sql.offset(len as isize) = '\0' as i32 as libc::c_char;
        return sql.offset(len as isize).offset(1 as libc::c_int as isize);
    }
    return parse_error(index);
}
unsafe extern "C" fn skip_until_const(
    mut index: Oid,
    mut sql: *mut libc::c_char,
    mut what: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    pos = strstr(sql, what);
    if !pos.is_null() {
        let mut len: size_t = 0;
        len = strlen(what);
        *pos.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        return pos.offset(len as isize).offset(1 as libc::c_int as isize);
    }
    return parse_error(index);
}
unsafe extern "C" fn skip_ident(
    mut index: Oid,
    mut sql: *mut libc::c_char,
) -> *mut libc::c_char {
    while *sql as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*sql as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        sql = sql.offset(1);
        sql;
    }
    if *sql as libc::c_int == '"' as i32 {
        sql = sql.offset(1);
        sql;
        loop {
            let mut end: *mut libc::c_char = strchr(sql, '"' as i32);
            if end.is_null() {
                return parse_error(index)
            } else if *end.offset(1 as libc::c_int as isize) as libc::c_int != '"' as i32
            {
                *end.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                return end.offset(2 as libc::c_int as isize);
            } else {
                sql = end.offset(2 as libc::c_int as isize);
            }
        }
    } else {
        while *sql as libc::c_int != 0
            && (*sql as libc::c_uchar as libc::c_int & 0x80 as libc::c_int != 0
                || *(*__ctype_b_loc())
                    .offset(*sql as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *sql as libc::c_int == '_' as i32)
        {
            sql = sql.offset(1);
            sql;
        }
        *sql = '\0' as i32 as libc::c_char;
        return sql.offset(1 as libc::c_int as isize);
    };
}
unsafe extern "C" fn skip_until(
    mut index: Oid,
    mut sql: *mut libc::c_char,
    mut end: libc::c_char,
) -> *mut libc::c_char {
    let mut instr: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut nopen: libc::c_int = 0 as libc::c_int;
    while *sql as libc::c_int != 0
        && (nopen > 0 as libc::c_int || instr as libc::c_int != 0 as libc::c_int
            || *sql as libc::c_int != end as libc::c_int)
    {
        if instr != 0 {
            if *sql.offset(0 as libc::c_int as isize) as libc::c_int
                == instr as libc::c_int
            {
                if *sql.offset(1 as libc::c_int as isize) as libc::c_int
                    == instr as libc::c_int
                {
                    sql = sql.offset(1);
                    sql;
                } else {
                    instr = 0 as libc::c_int as libc::c_char;
                }
            } else if *sql.offset(0 as libc::c_int as isize) as libc::c_int
                == '\\' as i32
            {
                sql = sql.offset(1);
                sql;
            }
        } else {
            match *sql.offset(0 as libc::c_int as isize) as libc::c_int {
                40 => {
                    nopen += 1;
                    nopen;
                }
                41 => {
                    nopen -= 1;
                    nopen;
                }
                39 | 34 => {
                    instr = *sql.offset(0 as libc::c_int as isize);
                }
                _ => {}
            }
        }
        sql = sql.offset(1);
        sql;
    }
    if nopen == 0 as libc::c_int && instr as libc::c_int == 0 as libc::c_int {
        if *sql != 0 {
            *sql = '\0' as i32 as libc::c_char;
            return sql.offset(1 as libc::c_int as isize);
        } else {
            return 0 as *mut libc::c_char
        }
    }
    return parse_error(index);
}
unsafe extern "C" fn parse_indexdef(
    mut stmt: *mut IndexDef,
    mut index: Oid,
    mut table: Oid,
) {
    let mut sql: *mut libc::c_char = pg_get_indexdef_string(index);
    let mut idxname: *const libc::c_char = get_quoted_relname(index);
    let mut tblname: *const libc::c_char = get_relation_name(table);
    let mut limit: *const libc::c_char = strchr(sql, '\0' as i32);
    (*stmt).create = sql;
    sql = skip_const(
        index,
        sql,
        b"CREATE INDEX\0" as *const u8 as *const libc::c_char,
        b"CREATE UNIQUE INDEX\0" as *const u8 as *const libc::c_char,
    );
    (*stmt).index = sql;
    sql = skip_const(index, sql, idxname, 0 as *const libc::c_char);
    sql = skip_const(
        index,
        sql,
        b"ON\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    (*stmt).table = sql;
    sql = skip_const(index, sql, tblname, 0 as *const libc::c_char);
    sql = skip_const(
        index,
        sql,
        b"USING\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    (*stmt).type_0 = sql;
    sql = skip_ident(index, sql);
    sql = strchr(sql, '(' as i32);
    if sql.is_null() {
        parse_error(index);
    }
    sql = sql.offset(1);
    sql;
    (*stmt).columns = sql;
    sql = skip_until(index, sql, ')' as i32 as libc::c_char);
    if sql.is_null() {
        parse_error(index);
    }
    (*stmt).options = sql;
    (*stmt).tablespace = 0 as *mut libc::c_char;
    (*stmt).where_0 = 0 as *mut libc::c_char;
    if sql < limit as *mut libc::c_char
        && !(strstr(sql, b"TABLESPACE\0" as *const u8 as *const libc::c_char)).is_null()
    {
        sql = skip_until_const(
            index,
            sql,
            b"TABLESPACE\0" as *const u8 as *const libc::c_char,
        );
        (*stmt).tablespace = sql;
        sql = skip_ident(index, sql);
    }
    if sql < limit as *mut libc::c_char
        && !(strstr(sql, b"WHERE\0" as *const u8 as *const libc::c_char)).is_null()
    {
        sql = skip_until_const(
            index,
            sql,
            b"WHERE\0" as *const u8 as *const libc::c_char,
        );
        (*stmt).where_0 = sql;
    }
    let mut __errno_location: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        604 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.create  = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).create,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_0: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        605 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.index   = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).index,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_1: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        606 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.table   = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).table,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_2: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        607 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.type    = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).type_0,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_3: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        608 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.columns = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).columns,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_4: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        609 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.options = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).options,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_5: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        610 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.tspace  = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).tablespace,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
    let mut __errno_location_6: libc::c_int = 0;
    elog_start(
        b"repack.c\0" as *const u8 as *const libc::c_char,
        611 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_indexdef\0"))
            .as_ptr(),
    );
    elog_finish(
        13 as libc::c_int,
        b"indexdef.where   = %s\0" as *const u8 as *const libc::c_char,
        (*stmt).where_0,
    );
    if 0 != 0 && 13 as libc::c_int >= 20 as libc::c_int {
        unreachable!();
    }
}
unsafe extern "C" fn parse_indexdef_col(
    mut token: *mut libc::c_char,
    mut desc: *mut *mut libc::c_char,
    mut nulls: *mut *mut libc::c_char,
    mut collate: *mut *mut libc::c_char,
) {
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    pos = strstr(token, b" NULLS FIRST\0" as *const u8 as *const libc::c_char);
    if !pos.is_null() {
        *nulls = pos.offset(1 as libc::c_int as isize);
        *pos = '\0' as i32 as libc::c_char;
    } else {
        pos = strstr(token, b" NULLS LAST\0" as *const u8 as *const libc::c_char);
        if !pos.is_null() {
            *nulls = pos.offset(1 as libc::c_int as isize);
            *pos = '\0' as i32 as libc::c_char;
        }
    }
    pos = strstr(token, b" DESC\0" as *const u8 as *const libc::c_char);
    if !pos.is_null() {
        *desc = pos.offset(1 as libc::c_int as isize);
        *pos = '\0' as i32 as libc::c_char;
    }
    pos = strstr(token, b" COLLATE \0" as *const u8 as *const libc::c_char);
    if !pos.is_null() {
        *collate = pos.offset(1 as libc::c_int as isize);
        *pos = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn repack_get_order_by(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut index: Oid = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .value as Oid;
    let mut table: Oid = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
        .value as Oid;
    let mut stmt: IndexDef = IndexDef {
        create: 0 as *mut libc::c_char,
        index: 0 as *mut libc::c_char,
        table: 0 as *mut libc::c_char,
        type_0: 0 as *mut libc::c_char,
        columns: 0 as *mut libc::c_char,
        options: 0 as *mut libc::c_char,
        tablespace: 0 as *mut libc::c_char,
        where_0: 0 as *mut libc::c_char,
    };
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut indexRel: Relation = 0 as Relation;
    let mut nattr: libc::c_int = 0;
    parse_indexdef(&mut stmt, index, table);
    initStringInfo(&mut str);
    nattr = 0 as libc::c_int;
    next = stmt.columns;
    while !next.is_null() {
        let mut opcname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut coldesc: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut colnulls: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut colcollate: *mut libc::c_char = 0 as *mut libc::c_char;
        token = next;
        while *(*__ctype_b_loc()).offset(*token as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            token = token.offset(1);
            token;
        }
        next = skip_until(index, next, ',' as i32 as libc::c_char);
        parse_indexdef_col(token, &mut coldesc, &mut colnulls, &mut colcollate);
        opcname = skip_until(index, token, ' ' as i32 as libc::c_char);
        appendStringInfoString(&mut str, token);
        if !colcollate.is_null() {
            appendStringInfo(
                &mut str as *mut StringInfoData,
                b" %s\0" as *const u8 as *const libc::c_char,
                colcollate,
            );
        }
        if !coldesc.is_null() {
            appendStringInfo(
                &mut str as *mut StringInfoData,
                b" %s\0" as *const u8 as *const libc::c_char,
                coldesc,
            );
        }
        if !opcname.is_null() {
            let mut opclass: Oid = 0;
            let mut oprid: Oid = 0;
            let mut strategy: int16 = 1 as libc::c_int as int16;
            let mut opcintype: Oid = 0;
            let mut opfamily: Oid = 0;
            let mut tp: HeapTuple = 0 as *mut HeapTupleData;
            let mut opclassTup: Form_pg_opclass = 0 as *mut FormData_pg_opclass;
            opclass = OpclassnameGetOpcid(403 as libc::c_int as Oid, opcname);
            tp = SearchSysCache(
                CLAOID as libc::c_int,
                opclass as Datum,
                0 as libc::c_int as Datum,
                0 as libc::c_int as Datum,
                0 as libc::c_int as Datum,
            );
            if (tp as *const libc::c_void).is_null() {
                let mut __errno_location: libc::c_int = 0;
                elog_start(
                    b"repack.c\0" as *const u8 as *const libc::c_char,
                    712 as libc::c_int,
                    (*::std::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"repack_get_order_by\0"))
                        .as_ptr(),
                );
                elog_finish(
                    20 as libc::c_int,
                    b"cache lookup failed for opclass %u\0" as *const u8
                        as *const libc::c_char,
                    opclass,
                );
                if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                    unreachable!();
                }
            }
            opclassTup = ((*tp).t_data as *mut libc::c_char)
                .offset((*(*tp).t_data).t_hoff as libc::c_int as isize)
                as Form_pg_opclass;
            opfamily = (*opclassTup).opcfamily;
            opcintype = (*opclassTup).opcintype;
            ReleaseSysCache(tp);
            if !(opcintype != 0 as libc::c_int as Oid) {
                if indexRel.is_null() {
                    indexRel = index_open(index, 0 as libc::c_int);
                }
                opcintype = (*((*(*indexRel).rd_att).attrs)
                    .as_mut_ptr()
                    .offset(nattr as isize))
                    .atttypid;
            }
            oprid = get_opfamily_member(opfamily, opcintype, opcintype, strategy);
            if !(oprid != 0 as libc::c_int as Oid) {
                let mut __errno_location_0: libc::c_int = 0;
                elog_start(
                    b"repack.c\0" as *const u8 as *const libc::c_char,
                    733 as libc::c_int,
                    (*::std::mem::transmute::<
                        &[u8; 20],
                        &[libc::c_char; 20],
                    >(b"repack_get_order_by\0"))
                        .as_ptr(),
                );
                elog_finish(
                    20 as libc::c_int,
                    b"missing operator %d(%u,%u) in opfamily %u\0" as *const u8
                        as *const libc::c_char,
                    strategy as libc::c_int,
                    opcintype,
                    opcintype,
                    opfamily,
                );
                if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                    unreachable!();
                }
            }
            *opcname.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            appendStringInfo(
                &mut str as *mut StringInfoData,
                b" USING %s\0" as *const u8 as *const libc::c_char,
                get_opname(oprid),
            );
        }
        if !colnulls.is_null() {
            appendStringInfo(
                &mut str as *mut StringInfoData,
                b" %s\0" as *const u8 as *const libc::c_char,
                colnulls,
            );
        }
        if !next.is_null() {
            appendStringInfoString(
                &mut str,
                b", \0" as *const u8 as *const libc::c_char,
            );
        }
        nattr += 1;
        nattr;
    }
    if !indexRel.is_null() {
        index_close(indexRel, 0 as libc::c_int);
    }
    return cstring_to_text(str.data) as Datum;
}
pub unsafe extern "C" fn repack_indexdef(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut index: Oid = 0;
    let mut table: Oid = 0;
    let mut tablespace: Name = 0 as Name;
    let mut stmt: IndexDef = IndexDef {
        create: 0 as *mut libc::c_char,
        index: 0 as *mut libc::c_char,
        table: 0 as *mut libc::c_char,
        type_0: 0 as *mut libc::c_char,
        columns: 0 as *mut libc::c_char,
        options: 0 as *mut libc::c_char,
        tablespace: 0 as *mut libc::c_char,
        where_0: 0 as *mut libc::c_char,
    };
    let mut str: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut concurrent_index: bool = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize))
        .value != 0 as libc::c_int as libc::c_ulong;
    if (*((*fcinfo).args).as_mut_ptr().offset(0 as libc::c_int as isize)).isnull
        as libc::c_int != 0
        || (*((*fcinfo).args).as_mut_ptr().offset(1 as libc::c_int as isize)).isnull
            as libc::c_int != 0
    {
        (*fcinfo).isnull = 1 as libc::c_int != 0;
        return 0 as libc::c_int as Datum;
    }
    index = (*((*fcinfo).args).as_mut_ptr().offset(0 as libc::c_int as isize)).value
        as Oid;
    table = (*((*fcinfo).args).as_mut_ptr().offset(1 as libc::c_int as isize)).value
        as Oid;
    if !(*((*fcinfo).args).as_mut_ptr().offset(2 as libc::c_int as isize)).isnull {
        tablespace = (*((*fcinfo).args).as_mut_ptr().offset(2 as libc::c_int as isize))
            .value as Pointer as Name;
    }
    parse_indexdef(&mut stmt, index, table);
    initStringInfo(&mut str);
    if concurrent_index {
        appendStringInfo(
            &mut str as *mut StringInfoData,
            b"%s CONCURRENTLY index_%u ON %s USING %s (%s)%s\0" as *const u8
                as *const libc::c_char,
            stmt.create,
            index,
            stmt.table,
            stmt.type_0,
            stmt.columns,
            stmt.options,
        );
    } else {
        appendStringInfo(
            &mut str as *mut StringInfoData,
            b"%s index_%u ON repack.table_%u USING %s (%s)%s\0" as *const u8
                as *const libc::c_char,
            stmt.create,
            index,
            table,
            stmt.type_0,
            stmt.columns,
            stmt.options,
        );
    }
    if !tablespace.is_null() || !(stmt.tablespace).is_null() {
        appendStringInfo(
            &mut str as *mut StringInfoData,
            b" TABLESPACE %s\0" as *const u8 as *const libc::c_char,
            if !tablespace.is_null() {
                ((*tablespace).data).as_mut_ptr()
            } else {
                stmt.tablespace
            },
        );
    }
    if !(stmt.where_0).is_null() {
        appendStringInfo(
            &mut str as *mut StringInfoData,
            b" WHERE %s\0" as *const u8 as *const libc::c_char,
            stmt.where_0,
        );
    }
    return cstring_to_text(str.data) as Datum;
}
unsafe extern "C" fn getoid(
    mut tuple: HeapTuple,
    mut desc: TupleDesc,
    mut column: libc::c_int,
) -> Oid {
    let mut isnull: bool = false;
    let mut datum: Datum = SPI_getbinval(tuple, desc, column, &mut isnull);
    return if isnull as libc::c_int != 0 {
        0 as libc::c_int as Oid
    } else {
        datum as Oid
    };
}
pub unsafe extern "C" fn repack_swap(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut oid: Oid = (*((*fcinfo).args).as_mut_ptr().offset(0 as libc::c_int as isize))
        .value as Oid;
    let mut relname: *const libc::c_char = get_quoted_relname(oid);
    let mut nspname: *const libc::c_char = get_quoted_nspname(oid);
    let mut argtypes: [Oid; 1] = [26 as libc::c_int as Oid];
    let mut nulls: [bool; 1] = [0 as libc::c_int != 0];
    let mut values: [Datum; 1] = [0; 1];
    let mut tuptable: *mut SPITupleTable = 0 as *mut SPITupleTable;
    let mut desc: TupleDesc = 0 as *mut TupleDescData;
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    let mut records: uint32 = 0;
    let mut i: uint32 = 0;
    let mut reltoastrelid1: Oid = 0;
    let mut reltoastidxid1: Oid = 0;
    let mut oid2: Oid = 0;
    let mut reltoastrelid2: Oid = 0;
    let mut reltoastidxid2: Oid = 0;
    let mut owner1: Oid = 0;
    let mut owner2: Oid = 0;
    must_be_superuser(b"repack_swap\0" as *const u8 as *const libc::c_char);
    repack_init();
    values[0 as libc::c_int as usize] = oid as Datum;
    execute_with_args(
        5 as libc::c_int,
        b"SELECT X.reltoastrelid, TX.indexrelid, X.relowner,       Y.oid, Y.reltoastrelid, TY.indexrelid, Y.relowner  FROM pg_catalog.pg_class X LEFT JOIN pg_catalog.pg_index TX         ON X.reltoastrelid = TX.indrelid AND TX.indisvalid,       pg_catalog.pg_class Y LEFT JOIN pg_catalog.pg_index TY         ON Y.reltoastrelid = TY.indrelid AND TY.indisvalid WHERE X.oid = $1   AND Y.oid = ('repack.table_' || X.oid)::regclass\0"
            as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        argtypes.as_mut_ptr(),
        values.as_mut_ptr(),
        nulls.as_mut_ptr() as *const bool,
    );
    tuptable = SPI_tuptable;
    desc = (*tuptable).tupdesc;
    records = SPI_processed as uint32;
    if records == 0 as libc::c_int as libc::c_uint {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            869 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"repack_swap\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"repack_swap : no swap target\0" as *const u8 as *const libc::c_char,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    tuple = *((*tuptable).vals).offset(0 as libc::c_int as isize);
    reltoastrelid1 = getoid(tuple, desc, 1 as libc::c_int);
    reltoastidxid1 = getoid(tuple, desc, 2 as libc::c_int);
    owner1 = getoid(tuple, desc, 3 as libc::c_int);
    oid2 = getoid(tuple, desc, 4 as libc::c_int);
    reltoastrelid2 = getoid(tuple, desc, 5 as libc::c_int);
    reltoastidxid2 = getoid(tuple, desc, 6 as libc::c_int);
    owner2 = getoid(tuple, desc, 7 as libc::c_int);
    if owner1 != owner2 {
        ATExecChangeOwner(oid2, owner1, 1 as libc::c_int != 0, 8 as libc::c_int);
        CommandCounterIncrement();
    }
    swap_heap_or_index_files(oid, oid2);
    CommandCounterIncrement();
    values[0 as libc::c_int as usize] = oid as Datum;
    execute_with_args(
        5 as libc::c_int,
        b"SELECT X.oid, Y.oid  FROM pg_catalog.pg_index I,       pg_catalog.pg_class X,       pg_catalog.pg_class Y WHERE I.indrelid = $1   AND I.indexrelid = X.oid   AND I.indisvalid   AND Y.oid = ('repack.index_' || X.oid)::regclass\0"
            as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        argtypes.as_mut_ptr(),
        values.as_mut_ptr(),
        nulls.as_mut_ptr() as *const bool,
    );
    tuptable = SPI_tuptable;
    desc = (*tuptable).tupdesc;
    records = SPI_processed as uint32;
    i = 0 as libc::c_int as uint32;
    while i < records {
        let mut idx1: Oid = 0;
        let mut idx2: Oid = 0;
        tuple = *((*tuptable).vals).offset(i as isize);
        idx1 = getoid(tuple, desc, 1 as libc::c_int);
        idx2 = getoid(tuple, desc, 2 as libc::c_int);
        swap_heap_or_index_files(idx1, idx2);
        CommandCounterIncrement();
        i = i.wrapping_add(1);
        i;
    }
    if reltoastrelid1 == 0 as libc::c_int as Oid {
        if reltoastidxid1 != 0 as libc::c_int as Oid
            || reltoastrelid2 != 0 as libc::c_int as Oid
            || reltoastidxid2 != 0 as libc::c_int as Oid
        {
            let mut __errno_location_0: libc::c_int = 0;
            elog_start(
                b"repack.c\0" as *const u8 as *const libc::c_char,
                928 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 12],
                    &[libc::c_char; 12],
                >(b"repack_swap\0"))
                    .as_ptr(),
            );
            elog_finish(
                20 as libc::c_int,
                b"repack_swap : unexpected toast relations (T1=%u, I1=%u, T2=%u, I2=%u\0"
                    as *const u8 as *const libc::c_char,
                reltoastrelid1,
                reltoastidxid1,
                reltoastrelid2,
                reltoastidxid2,
            );
            if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                unreachable!();
            }
        }
    } else if reltoastrelid2 == 0 as libc::c_int as Oid {
        let mut name: [libc::c_char; 64] = [0; 64];
        if reltoastidxid1 == 0 as libc::c_int as Oid
            || reltoastidxid2 != 0 as libc::c_int as Oid
        {
            let mut __errno_location_1: libc::c_int = 0;
            elog_start(
                b"repack.c\0" as *const u8 as *const libc::c_char,
                938 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 12],
                    &[libc::c_char; 12],
                >(b"repack_swap\0"))
                    .as_ptr(),
            );
            elog_finish(
                20 as libc::c_int,
                b"repack_swap : unexpected toast relations (T1=%u, I1=%u, T2=%u, I2=%u\0"
                    as *const u8 as *const libc::c_char,
                reltoastrelid1,
                reltoastidxid1,
                reltoastrelid2,
                reltoastidxid2,
            );
            if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                unreachable!();
            }
        }
        pg_snprintf(
            name.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u\0" as *const u8 as *const libc::c_char,
            oid2,
        );
        RenameRelationInternal(
            reltoastrelid1,
            name.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        pg_snprintf(
            name.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u_index\0" as *const u8 as *const libc::c_char,
            oid2,
        );
        RenameRelationInternal(
            reltoastidxid1,
            name.as_mut_ptr(),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        CommandCounterIncrement();
    } else if reltoastrelid1 != 0 as libc::c_int as Oid {
        let mut name_0: [libc::c_char; 64] = [0; 64];
        let mut pid: libc::c_int = getpid();
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_pid%d\0" as *const u8 as *const libc::c_char,
            pid,
        );
        RenameRelationInternal(
            reltoastrelid1,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_pid%d_index\0" as *const u8 as *const libc::c_char,
            pid,
        );
        RenameRelationInternal(
            reltoastidxid1,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        CommandCounterIncrement();
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u\0" as *const u8 as *const libc::c_char,
            oid,
        );
        RenameRelationInternal(
            reltoastrelid2,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u_index\0" as *const u8 as *const libc::c_char,
            oid,
        );
        RenameRelationInternal(
            reltoastidxid2,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        CommandCounterIncrement();
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u\0" as *const u8 as *const libc::c_char,
            oid2,
        );
        RenameRelationInternal(
            reltoastrelid1,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        pg_snprintf(
            name_0.as_mut_ptr(),
            64 as libc::c_int as size_t,
            b"pg_toast_%u_index\0" as *const u8 as *const libc::c_char,
            oid2,
        );
        RenameRelationInternal(
            reltoastidxid1,
            name_0.as_mut_ptr(),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
        CommandCounterIncrement();
    }
    execute_with_format(
        4 as libc::c_int,
        b"DROP TRIGGER IF EXISTS repack_trigger ON %s.%s CASCADE\0" as *const u8
            as *const libc::c_char,
        nspname,
        relname,
    );
    SPI_finish();
    return 0 as libc::c_int as Datum;
}
pub unsafe extern "C" fn repack_drop(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut oid: Oid = (*((*fcinfo).args).as_mut_ptr().offset(0 as libc::c_int as isize))
        .value as Oid;
    let mut numobj: libc::c_int = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
        .value as int32;
    let mut relname: *const libc::c_char = get_quoted_relname(oid);
    let mut nspname: *const libc::c_char = get_quoted_nspname(oid);
    if !(!relname.is_null() && !nspname.is_null()) {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1004 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"repack_drop\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"table name not found for OID %u\0" as *const u8 as *const libc::c_char,
            oid,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
        return 0 as libc::c_int as Datum;
    }
    must_be_superuser(b"repack_drop\0" as *const u8 as *const libc::c_char);
    repack_init();
    if numobj > 0 as libc::c_int {
        execute_with_format(
            4 as libc::c_int,
            b"LOCK TABLE %s.%s IN ACCESS EXCLUSIVE MODE\0" as *const u8
                as *const libc::c_char,
            nspname,
            relname,
        );
    }
    if numobj > 1 as libc::c_int {
        execute_with_format(
            4 as libc::c_int,
            b"DROP TABLE IF EXISTS repack.log_%u CASCADE\0" as *const u8
                as *const libc::c_char,
            oid,
        );
        numobj -= 1;
        numobj;
    }
    if numobj > 0 as libc::c_int {
        execute_with_format(
            4 as libc::c_int,
            b"DROP TYPE IF EXISTS repack.pk_%u\0" as *const u8 as *const libc::c_char,
            oid,
        );
        numobj -= 1;
        numobj;
    }
    if numobj > 0 as libc::c_int {
        execute_with_format(
            4 as libc::c_int,
            b"DROP TRIGGER IF EXISTS repack_trigger ON %s.%s CASCADE\0" as *const u8
                as *const libc::c_char,
            nspname,
            relname,
        );
        numobj -= 1;
        numobj;
    }
    if numobj > 0 as libc::c_int {
        execute_with_format(
            4 as libc::c_int,
            b"DROP TABLE IF EXISTS repack.table_%u CASCADE\0" as *const u8
                as *const libc::c_char,
            oid,
        );
        numobj -= 1;
        numobj;
    }
    SPI_finish();
    return 0 as libc::c_int as Datum;
}
pub unsafe extern "C" fn repack_disable_autovacuum(
    mut fcinfo: FunctionCallInfo,
) -> Datum {
    let mut oid: Oid = (*((*fcinfo).args).as_mut_ptr().offset(0 as libc::c_int as isize))
        .value as Oid;
    repack_init();
    execute_with_format(
        4 as libc::c_int,
        b"ALTER TABLE %s SET (autovacuum_enabled = off)\0" as *const u8
            as *const libc::c_char,
        get_relation_name(oid),
    );
    SPI_finish();
    return 0 as libc::c_int as Datum;
}
unsafe extern "C" fn repack_init() {
    let mut ret: libc::c_int = SPI_connect();
    if ret != 1 as libc::c_int {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1121 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"repack_init\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"pg_repack: SPI_connect returned %d\0" as *const u8 as *const libc::c_char,
            ret,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
}
unsafe extern "C" fn repack_prepare(
    mut src: *const libc::c_char,
    mut nargs: libc::c_int,
    mut argtypes: *mut Oid,
) -> SPIPlanPtr {
    let mut plan: SPIPlanPtr = SPI_prepare(src, nargs, argtypes);
    if plan.is_null() {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1130 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"repack_prepare\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"pg_repack: repack_prepare failed (code=%d, query=%s)\0" as *const u8
                as *const libc::c_char,
            SPI_result,
            src,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    return plan;
}
unsafe extern "C" fn get_quoted_relname(mut oid: Oid) -> *const libc::c_char {
    let mut relname: *const libc::c_char = get_rel_name(oid);
    return if !relname.is_null() {
        quote_identifier(relname)
    } else {
        0 as *const libc::c_char
    };
}
unsafe extern "C" fn get_quoted_nspname(mut oid: Oid) -> *const libc::c_char {
    let mut nspname: *const libc::c_char = get_namespace_name(get_rel_namespace(oid));
    return if !nspname.is_null() {
        quote_identifier(nspname)
    } else {
        0 as *const libc::c_char
    };
}
unsafe extern "C" fn swap_heap_or_index_files(mut r1: Oid, mut r2: Oid) {
    let mut relRelation: Relation = 0 as *mut RelationData;
    let mut reltup1: HeapTuple = 0 as *mut HeapTupleData;
    let mut reltup2: HeapTuple = 0 as *mut HeapTupleData;
    let mut relform1: Form_pg_class = 0 as *mut FormData_pg_class;
    let mut relform2: Form_pg_class = 0 as *mut FormData_pg_class;
    let mut swaptemp: Oid = 0;
    let mut indstate: CatalogIndexState = 0 as *mut ResultRelInfo;
    relRelation = table_open(1259 as libc::c_int as Oid, 3 as libc::c_int);
    reltup1 = SearchSysCacheCopy(
        RELOID as libc::c_int,
        r1 as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
    );
    if (reltup1 as *const libc::c_void).is_null() {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1172 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"swap_heap_or_index_files\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"cache lookup failed for relation %u\0" as *const u8 as *const libc::c_char,
            r1,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    relform1 = ((*reltup1).t_data as *mut libc::c_char)
        .offset((*(*reltup1).t_data).t_hoff as libc::c_int as isize) as Form_pg_class;
    reltup2 = SearchSysCacheCopy(
        RELOID as libc::c_int,
        r2 as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
    );
    if (reltup2 as *const libc::c_void).is_null() {
        let mut __errno_location_0: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1177 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"swap_heap_or_index_files\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"cache lookup failed for relation %u\0" as *const u8 as *const libc::c_char,
            r2,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    relform2 = ((*reltup2).t_data as *mut libc::c_char)
        .offset((*(*reltup2).t_data).t_hoff as libc::c_int as isize) as Form_pg_class;
    swaptemp = (*relform1).relfilenode;
    (*relform1).relfilenode = (*relform2).relfilenode;
    (*relform2).relfilenode = swaptemp;
    swaptemp = (*relform1).reltablespace;
    (*relform1).reltablespace = (*relform2).reltablespace;
    (*relform2).reltablespace = swaptemp;
    swaptemp = (*relform1).reltoastrelid;
    (*relform1).reltoastrelid = (*relform2).reltoastrelid;
    (*relform2).reltoastrelid = swaptemp;
    if (*relform1).relfrozenxid >= 3 as libc::c_int as TransactionId {
        if TransactionIdFollows((*relform1).relfrozenxid, (*relform2).relfrozenxid) {
            (*relform1).relfrozenxid = (*relform2).relfrozenxid;
        } else {
            (*relform2).relfrozenxid = (*relform1).relfrozenxid;
        }
    }
    let mut swap_pages: int32 = 0;
    let mut swap_tuples: float4 = 0.;
    swap_pages = (*relform1).relpages;
    (*relform1).relpages = (*relform2).relpages;
    (*relform2).relpages = swap_pages;
    swap_tuples = (*relform1).reltuples;
    (*relform1).reltuples = (*relform2).reltuples;
    (*relform2).reltuples = swap_tuples;
    indstate = CatalogOpenIndexes(relRelation);
    CatalogTupleUpdateWithInfo(relRelation, &mut (*reltup1).t_self, reltup1, indstate);
    CatalogTupleUpdateWithInfo(relRelation, &mut (*reltup2).t_self, reltup2, indstate);
    CatalogCloseIndexes(indstate);
    if (*relform1).reltoastrelid != 0 || (*relform2).reltoastrelid != 0 {
        let mut baseobject: ObjectAddress = ObjectAddress {
            classId: 0,
            objectId: 0,
            objectSubId: 0,
        };
        let mut toastobject: ObjectAddress = ObjectAddress {
            classId: 0,
            objectId: 0,
            objectSubId: 0,
        };
        let mut count: libc::c_long = 0;
        if (*relform1).reltoastrelid != 0 {
            count = deleteDependencyRecordsFor(
                1259 as libc::c_int as Oid,
                (*relform1).reltoastrelid,
                0 as libc::c_int != 0,
            );
            if count != 1 as libc::c_int as libc::c_long {
                let mut __errno_location_1: libc::c_int = 0;
                elog_start(
                    b"repack.c\0" as *const u8 as *const libc::c_char,
                    1270 as libc::c_int,
                    (*::std::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"swap_heap_or_index_files\0"))
                        .as_ptr(),
                );
                elog_finish(
                    20 as libc::c_int,
                    b"expected one dependency record for TOAST table, found %ld\0"
                        as *const u8 as *const libc::c_char,
                    count,
                );
                if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                    unreachable!();
                }
            }
        }
        if (*relform2).reltoastrelid != 0 {
            count = deleteDependencyRecordsFor(
                1259 as libc::c_int as Oid,
                (*relform2).reltoastrelid,
                0 as libc::c_int != 0,
            );
            if count != 1 as libc::c_int as libc::c_long {
                let mut __errno_location_2: libc::c_int = 0;
                elog_start(
                    b"repack.c\0" as *const u8 as *const libc::c_char,
                    1279 as libc::c_int,
                    (*::std::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"swap_heap_or_index_files\0"))
                        .as_ptr(),
                );
                elog_finish(
                    20 as libc::c_int,
                    b"expected one dependency record for TOAST table, found %ld\0"
                        as *const u8 as *const libc::c_char,
                    count,
                );
                if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
                    unreachable!();
                }
            }
        }
        baseobject.classId = 1259 as libc::c_int as Oid;
        baseobject.objectSubId = 0 as libc::c_int;
        toastobject.classId = 1259 as libc::c_int as Oid;
        toastobject.objectSubId = 0 as libc::c_int;
        if (*relform1).reltoastrelid != 0 {
            baseobject.objectId = r1;
            toastobject.objectId = (*relform1).reltoastrelid;
            recordDependencyOn(&mut toastobject, &mut baseobject, DEPENDENCY_INTERNAL);
        }
        if (*relform2).reltoastrelid != 0 {
            baseobject.objectId = r2;
            toastobject.objectId = (*relform2).reltoastrelid;
            recordDependencyOn(&mut toastobject, &mut baseobject, DEPENDENCY_INTERNAL);
        }
    }
    RelationForgetRelation(r1);
    RelationForgetRelation(r2);
    heap_freetuple(reltup1);
    heap_freetuple(reltup2);
    table_close(relRelation, 3 as libc::c_int);
}
pub unsafe extern "C" fn repack_index_swap(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut orig_idx_oid: Oid = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .value as Oid;
    let mut repacked_idx_oid: Oid = 0;
    let mut str: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut tuptable: *mut SPITupleTable = 0 as *mut SPITupleTable;
    let mut desc: TupleDesc = 0 as *mut TupleDescData;
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    must_be_superuser(b"repack_index_swap\0" as *const u8 as *const libc::c_char);
    repack_init();
    initStringInfo(&mut str);
    appendStringInfo(
        &mut str as *mut StringInfoData,
        b"SELECT oid FROM pg_class WHERE relname = 'index_%u' AND relkind = 'i'\0"
            as *const u8 as *const libc::c_char,
        orig_idx_oid,
    );
    execute(5 as libc::c_int, str.data);
    if SPI_processed != 1 as libc::c_int as libc::c_ulong {
        let mut __errno_location: libc::c_int = 0;
        elog_start(
            b"repack.c\0" as *const u8 as *const libc::c_char,
            1364 as libc::c_int,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"repack_index_swap\0"))
                .as_ptr(),
        );
        elog_finish(
            20 as libc::c_int,
            b"Could not find index 'index_%u', found %lu matches\0" as *const u8
                as *const libc::c_char,
            orig_idx_oid,
            SPI_processed,
        );
        if 0 != 0 && 20 as libc::c_int >= 20 as libc::c_int {
            unreachable!();
        }
    }
    tuptable = SPI_tuptable;
    desc = (*tuptable).tupdesc;
    tuple = *((*tuptable).vals).offset(0 as libc::c_int as isize);
    repacked_idx_oid = getoid(tuple, desc, 1 as libc::c_int);
    swap_heap_or_index_files(orig_idx_oid, repacked_idx_oid);
    SPI_finish();
    return 0 as libc::c_int as Datum;
}
pub unsafe extern "C" fn repack_get_table_and_inheritors(
    mut fcinfo: FunctionCallInfo,
) -> Datum {
    let mut parent: Oid = (*((*fcinfo).args)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .value as Oid;
    let mut relations: *mut List = 0 as *mut List;
    let mut relations_array: *mut Datum = 0 as *mut Datum;
    let mut relations_array_size: libc::c_int = 0;
    let mut result: *mut ArrayType = 0 as *mut ArrayType;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0;
    LockRelationOid(parent, 1 as libc::c_int);
    if !SearchSysCacheExists(
        RELOID as libc::c_int,
        parent as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
        0 as libc::c_int as Datum,
    ) {
        return construct_empty_array(26 as libc::c_int as Oid) as Datum;
    }
    relations = find_all_inheritors(parent, 1 as libc::c_int, 0 as *mut *mut List);
    relations_array_size = list_length(relations);
    if relations_array_size == 0 as libc::c_int {
        return construct_empty_array(26 as libc::c_int as Oid) as Datum;
    }
    relations_array = palloc(
        (relations_array_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Datum>() as libc::c_ulong),
    ) as *mut Datum;
    i = 0 as libc::c_int;
    lc = list_head(relations);
    while !lc.is_null() {
        let fresh0 = i;
        i = i + 1;
        *relations_array.offset(fresh0 as isize) = (*lc).data.oid_value as Datum;
        lc = (*lc).next;
    }
    result = construct_array(
        relations_array,
        relations_array_size,
        26 as libc::c_int as Oid,
        ::std::mem::size_of::<Oid>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int != 0,
        'i' as i32 as libc::c_char,
    );
    pfree(relations_array as *mut libc::c_void);
    return result as Datum;
}
