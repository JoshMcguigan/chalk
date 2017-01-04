use ast::*;
use lalrpop_intern::intern;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Goals {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22as_22(&'input str),
        Term_22for_22(&'input str),
        Term_22impl_22(&'input str),
        Term_22struct_22(&'input str),
        Term_22trait_22(&'input str),
        Term_22type_22(&'input str),
        Term_22where_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(Vec<Identifier>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(::std::option::Option<Vec<Identifier>>),
        Nt_28_29(()),
        Nt_40L(usize),
        Nt_40R(usize),
        NtAngle_3cId_3e(Vec<Identifier>),
        NtAngle_3cTy_3e(Vec<Ty>),
        NtAssocTyName(Identifier),
        NtAssocTyName_2a(::std::vec::Vec<Identifier>),
        NtAssocTyName_2b(::std::vec::Vec<Identifier>),
        NtAssocTyValue(AssocTyValue),
        NtAssocTyValue_2a(::std::vec::Vec<AssocTyValue>),
        NtAssocTyValue_2b(::std::vec::Vec<AssocTyValue>),
        NtComma1_3cId_3e(Vec<Identifier>),
        NtComma1_3cId_3e_3f(::std::option::Option<Vec<Identifier>>),
        NtComma1_3cTy_3e(Vec<Ty>),
        NtComma1_3cTy_3e_3f(::std::option::Option<Vec<Ty>>),
        NtComma1_3cWhereClause_3e(Vec<WhereClause>),
        NtComma1_3cWhereClause_3e_3f(::std::option::Option<Vec<WhereClause>>),
        NtComma_3cId_3e(Vec<Identifier>),
        NtComma_3cTy_3e(Vec<Ty>),
        NtComma_3cWhereClause_3e(Vec<WhereClause>),
        NtGoals(Goals),
        NtId(Identifier),
        NtImpl(Impl),
        NtItem(Item),
        NtItem_2a(::std::vec::Vec<Item>),
        NtItem_2b(::std::vec::Vec<Item>),
        NtProgram(Program),
        NtProjectionTy(ProjectionTy),
        NtStructDefn(StructDefn),
        NtTraitDefn(TraitDefn),
        NtTraitRef_3c_22_3a_22_3e(TraitRef),
        NtTraitRef_3c_22as_22_3e(TraitRef),
        NtTy(Ty),
        NtWhereClause(WhereClause),
        NtWhereClauses(Vec<WhereClause>),
        Nt____Goals(Goals),
        Nt____Program(Program),
        Nt____Ty(Ty),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        -38, 0, 0, 0, 10, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 1
        -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -59, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -61, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 10
        0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 13
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 14
        0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 16
        0, 0, 0, 0, 38, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 21
        0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -34, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0,
        // State 23
        -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -35, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        46, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -59, 0, 0, 0, 48, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        -25, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 30
        -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -59, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 35
        -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -10, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 38
        0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        -33, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        59, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -41, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 46
        0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 53
        46, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0,
        // State 55
        0, 0, 0, 0, 71, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0,
        // State 59
        -38, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 60
        -26, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        46, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        46, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        46, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        -36, 0, 0, 0, 30, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0,
        // State 72
        -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        -60, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0,
        // State 76
        -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 0,
        // State 78
        -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        46, 0, 0, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -38,
        -37,
        -40,
        -66,
        0,
        0,
        -62,
        0,
        -29,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -30,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -59,
        -61,
        -63,
        0,
        -41,
        -10,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -57,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -38,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -39,
        0,
        0,
        -60,
        0,
        -9,
        0,
        0,
        0,
        0,
        -53,
        -41,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 4, 5, 0, 0, 0, 0, 0, 6, 0, 0, 7, 0, 8, 9, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 18, 0, 0, 0, 19, 20, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 7, 0, 8, 24, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 26, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 18, 0, 0, 0, 41, 20, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 43, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 18, 0, 0, 0, 49, 20, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 18, 0, 0, 0, 51, 20, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 54, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 62, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 64, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 66, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 74, 0, 5, 0, 0, 0, 0, 0, 6, 0, 0, 7, 0, 8, 9, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 80, 0, 0, 27, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_Goals<
        'input,
    >(
        input: &'input str,
    ) -> Result<Goals, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (17, _) if true => 17,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_3a_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3d_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22as_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22for_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22impl_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22struct_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22trait_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22type_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22where_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Goals,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("for" "<" <Comma<Id>> ">") = "for", "<", Comma<Id>, ">" => ActionFn(39);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__nt), __end));
                0
            }
            2 => {
                // ("for" "<" <Comma<Id>> ">")? = "for", "<", Comma<Id>, ">" => ActionFn(61);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action61::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("for" "<" <Comma<Id>> ">")? =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            4 => {
                // () =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_29(__nt), __end));
                2
            }
            5 => {
                // @L =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                3
            }
            6 => {
                // @R =  => ActionFn(22);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action22::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40R(__nt), __end));
                4
            }
            7 => {
                // Angle<Id> = "<", Comma<Id>, ">" => ActionFn(34);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            8 => {
                // Angle<Id> =  => ActionFn(64);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action64::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            9 => {
                // Angle<Ty> = "<", Comma<Ty>, ">" => ActionFn(30);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            10 => {
                // Angle<Ty> =  => ActionFn(65);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action65::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            11 => {
                // AssocTyName = "type", Id, ";" => ActionFn(10);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssocTyName(__nt), __end));
                7
            }
            12 => {
                // AssocTyName* =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            13 => {
                // AssocTyName* = AssocTyName+ => ActionFn(33);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            14 => {
                // AssocTyName+ = AssocTyName => ActionFn(53);
                let __sym0 = __pop_NtAssocTyName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            15 => {
                // AssocTyName+ = AssocTyName+, AssocTyName => ActionFn(54);
                let __sym1 = __pop_NtAssocTyName(__symbols);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            16 => {
                // AssocTyValue = "type", Id, "=", Ty, ";" => ActionFn(12);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtTy(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtAssocTyValue(__nt), __end));
                10
            }
            17 => {
                // AssocTyValue* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            18 => {
                // AssocTyValue* = AssocTyValue+ => ActionFn(29);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            19 => {
                // AssocTyValue+ = AssocTyValue => ActionFn(55);
                let __sym0 = __pop_NtAssocTyValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            20 => {
                // AssocTyValue+ = AssocTyValue+, AssocTyValue => ActionFn(56);
                let __sym1 = __pop_NtAssocTyValue(__symbols);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            21 => {
                // Comma1<Id> = Id => ActionFn(47);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            22 => {
                // Comma1<Id> = Comma<Id>, ",", Id => ActionFn(48);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            23 => {
                // Comma1<Id>? = Comma1<Id> => ActionFn(45);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            24 => {
                // Comma1<Id>? =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            25 => {
                // Comma1<Ty> = Ty => ActionFn(59);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            26 => {
                // Comma1<Ty> = Comma<Ty>, ",", Ty => ActionFn(60);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            27 => {
                // Comma1<Ty>? = Comma1<Ty> => ActionFn(57);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            28 => {
                // Comma1<Ty>? =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            29 => {
                // Comma1<WhereClause> = WhereClause => ActionFn(51);
                let __sym0 = __pop_NtWhereClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            30 => {
                // Comma1<WhereClause> = Comma<WhereClause>, ",", WhereClause => ActionFn(52);
                let __sym2 = __pop_NtWhereClause(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            31 => {
                // Comma1<WhereClause>? = Comma1<WhereClause> => ActionFn(49);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            32 => {
                // Comma1<WhereClause>? =  => ActionFn(50);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action50::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            33 => {
                // Comma<Id> = Comma1<Id> => ActionFn(73);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            34 => {
                // Comma<Id> =  => ActionFn(74);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action74::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            35 => {
                // Comma<Ty> = Comma1<Ty> => ActionFn(75);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            36 => {
                // Comma<Ty> =  => ActionFn(76);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action76::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            37 => {
                // Comma<WhereClause> = Comma1<WhereClause> => ActionFn(77);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            38 => {
                // Comma<WhereClause> =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            39 => {
                // Goals = "for", "<", Comma<Id>, ">", Comma<WhereClause> => ActionFn(62);
                let __sym4 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action62::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            40 => {
                // Goals = Comma<WhereClause> => ActionFn(63);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            41 => {
                // Id = r#"([:alpha:]|_)([:alnum:]|_)*"# => ActionFn(68);
                let __sym0 = __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                23
            }
            42 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", "}" => ActionFn(71);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            43 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", AssocTyValue+, "}" => ActionFn(72);
                let __sym9 = __pop_Term_22_7d_22(__symbols);
                let __sym8 = __pop_NtAssocTyValue_2b(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym9.2.clone();
                let __nt = super::__action72::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
                let __states_len = __states.len();
                __states.truncate(__states_len - 10);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            44 => {
                // Item = StructDefn => ActionFn(4);
                let __sym0 = __pop_NtStructDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            45 => {
                // Item = TraitDefn => ActionFn(5);
                let __sym0 = __pop_NtTraitDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            46 => {
                // Item = Impl => ActionFn(6);
                let __sym0 = __pop_NtImpl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            47 => {
                // Item* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            48 => {
                // Item* = Item+ => ActionFn(42);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            49 => {
                // Item+ = Item => ActionFn(43);
                let __sym0 = __pop_NtItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            50 => {
                // Item+ = Item+, Item => ActionFn(44);
                let __sym1 = __pop_NtItem(__symbols);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            51 => {
                // Program =  => ActionFn(79);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action79::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            52 => {
                // Program = Item+ => ActionFn(80);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            53 => {
                // ProjectionTy = "<", TraitRef<"as">, ">", "::", Id => ActionFn(16);
                let __sym4 = __pop_NtId(__symbols);
                let __sym3 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtTraitRef_3c_22as_22_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtProjectionTy(__nt), __end));
                29
            }
            54 => {
                // StructDefn = "struct", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(8);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtStructDefn(__nt), __end));
                30
            }
            55 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(69);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            56 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", AssocTyName+, "}" => ActionFn(70);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtAssocTyName_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action70::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            57 => {
                // TraitRef<":"> = Ty, ":", Id, Angle<Ty> => ActionFn(24);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22_3a_22_3e(__nt), __end));
                32
            }
            58 => {
                // TraitRef<"as"> = Ty, "as", Id, Angle<Ty> => ActionFn(26);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22as_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22as_22_3e(__nt), __end));
                33
            }
            59 => {
                // Ty = Id => ActionFn(13);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            60 => {
                // Ty = Id, "<", Comma<Ty>, ">" => ActionFn(14);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            61 => {
                // Ty = ProjectionTy => ActionFn(15);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            62 => {
                // WhereClause = TraitRef<":"> => ActionFn(19);
                let __sym0 = __pop_NtTraitRef_3c_22_3a_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            63 => {
                // WhereClause = ProjectionTy, "==", Ty => ActionFn(20);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_3d_3d_22(__symbols);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            64 => {
                // WhereClauses = "where", Comma<WhereClause> => ActionFn(17);
                let __sym1 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym0 = __pop_Term_22where_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            65 => {
                // WhereClauses =  => ActionFn(66);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action66::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            66 => {
                // __Goals = Goals => ActionFn(1);
                let __sym0 = __pop_NtGoals(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            67 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                38
            }
            68 => {
                // __Ty = Ty => ActionFn(2);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ty(__nt), __end));
                39
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22as_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22as_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22for_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22for_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22impl_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22impl_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22trait_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22trait_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22type_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22type_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22where_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22where_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40R<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40R(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AssocTyValue, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Ty>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<WhereClause>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGoals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGoals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtImpl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Impl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtImpl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Item, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProjectionTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProjectionTy, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProjectionTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStructDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStructDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22_3a_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22_3a_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22as_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22as_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClause<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WhereClause, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClause(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClauses<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClauses(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Goals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Goals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ty<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ty(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Goals::parse_Goals;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22as_22(&'input str),
        Term_22for_22(&'input str),
        Term_22impl_22(&'input str),
        Term_22struct_22(&'input str),
        Term_22trait_22(&'input str),
        Term_22type_22(&'input str),
        Term_22where_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(Vec<Identifier>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(::std::option::Option<Vec<Identifier>>),
        Nt_28_29(()),
        Nt_40L(usize),
        Nt_40R(usize),
        NtAngle_3cId_3e(Vec<Identifier>),
        NtAngle_3cTy_3e(Vec<Ty>),
        NtAssocTyName(Identifier),
        NtAssocTyName_2a(::std::vec::Vec<Identifier>),
        NtAssocTyName_2b(::std::vec::Vec<Identifier>),
        NtAssocTyValue(AssocTyValue),
        NtAssocTyValue_2a(::std::vec::Vec<AssocTyValue>),
        NtAssocTyValue_2b(::std::vec::Vec<AssocTyValue>),
        NtComma1_3cId_3e(Vec<Identifier>),
        NtComma1_3cId_3e_3f(::std::option::Option<Vec<Identifier>>),
        NtComma1_3cTy_3e(Vec<Ty>),
        NtComma1_3cTy_3e_3f(::std::option::Option<Vec<Ty>>),
        NtComma1_3cWhereClause_3e(Vec<WhereClause>),
        NtComma1_3cWhereClause_3e_3f(::std::option::Option<Vec<WhereClause>>),
        NtComma_3cId_3e(Vec<Identifier>),
        NtComma_3cTy_3e(Vec<Ty>),
        NtComma_3cWhereClause_3e(Vec<WhereClause>),
        NtGoals(Goals),
        NtId(Identifier),
        NtImpl(Impl),
        NtItem(Item),
        NtItem_2a(::std::vec::Vec<Item>),
        NtItem_2b(::std::vec::Vec<Item>),
        NtProgram(Program),
        NtProjectionTy(ProjectionTy),
        NtStructDefn(StructDefn),
        NtTraitDefn(TraitDefn),
        NtTraitRef_3c_22_3a_22_3e(TraitRef),
        NtTraitRef_3c_22as_22_3e(TraitRef),
        NtTy(Ty),
        NtWhereClause(WhereClause),
        NtWhereClauses(Vec<WhereClause>),
        Nt____Goals(Goals),
        Nt____Program(Program),
        Nt____Ty(Ty),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, -49, -49, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, -44, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, -50, -50, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0,
        // State 12
        -34, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 13
        0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, -8, 0, 0, 0,
        // State 14
        0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, -41, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, -8, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 27, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -33, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        28, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -41, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, -65, 0, 0, 0,
        // State 23
        -34, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, -65, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0,
        // State 30
        -38, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 52, 0,
        // State 31
        28, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0,
        // State 34
        -35, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        59, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -59, 0, 0, 0, 61, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -25, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 40
        -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0,
        // State 43
        -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0,
        // State 44
        69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0,
        // State 45
        0, -59, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -61, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0,
        // State 48
        0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 51
        0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 77, 0, 0,
        // State 54
        0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, -59, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, -61, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, -65, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 58
        0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 61
        0, 0, 0, 0, 83, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 66
        0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, -54, -54, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 69
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 70
        0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 97, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, -55, -55, 0, 0, 0, 0, 0, 0,
        // State 77
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        -26, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        59, 0, 0, 0, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 83
        0, 0, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0,
        // State 87
        59, 0, 0, 0, 0, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        -59, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0,
        // State 89
        -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0,
        // State 90
        -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 92
        -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0,
        // State 93
        -10, 0, 0, 0, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0,
        // State 94
        0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, -56, -56, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        59, 0, 0, 0, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 120, 0, 0,
        // State 101
        0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        -60, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        59, 0, 0, 0, 0, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0,
        // State 105
        0, 0, 0, 0, 125, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0,
        // State 112
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, -11, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, -60, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 133, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 135, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 140, 0,
        // State 126
        59, 0, 0, 0, 0, 0, 0, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 142, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        59, 0, 0, 0, 0, 0, 0, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20, 0, 0,
        // State 132
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, -43, -43, 0, 0, 0, 0, 0, 0,
        // State 133
        0, 0, 0, 0, 0, 144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 135
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, -53, 0, 0, 0,
        // State 136
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, -41, 0, 0, 0,
        // State 137
        59, 0, 0, 0, 0, 0, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 138
        0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 140
        -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0,
        // State 141
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 0,
        // State 142
        -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0,
        // State 143
        0, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 0,
        // State 144
        0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0,
        // State 146
        -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0,
        // State 147
        0, 0, 0, -59, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, 154, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0,
        // State 151
        0, 0, 0, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        -36, 0, 0, 0, 40, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 153
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, -16, 0, 0,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        59, 0, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 159, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 157
        0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 0,
        // State 159
        0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -51,
        -46,
        -49,
        -52,
        -67,
        -44,
        -45,
        0,
        0,
        0,
        -50,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -54,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -55,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -42,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -43,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 11, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 20, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 32, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 36, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 45, 0, 46, 0, 0, 0, 0, 0, 47, 0, 0, 48, 0, 49, 50, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 64, 65, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 73, 65, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 74, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 80, 65, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 82, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 86, 65, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 47, 0, 0, 48, 0, 49, 87, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 88, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89, 0, 0, 0, 0, 0, 90, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 100, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 104, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 111, 65, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, 0, 0, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 127, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 129, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 138, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 132
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 135
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 137
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 138
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 142
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 148, 0, 0, 0, 0, 0, 149, 0, 0, 0, 0, 150, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 63, 0, 0, 0, 155, 65, 0, 0, 0, 0, 0,
        // State 151
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 156, 0, 0, 37, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 153
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 157
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 159
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Program, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (17, _) if true => 17,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_3a_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3d_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22as_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22for_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22impl_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22struct_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22trait_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22type_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22where_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("for" "<" <Comma<Id>> ">") = "for", "<", Comma<Id>, ">" => ActionFn(39);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__nt), __end));
                0
            }
            2 => {
                // ("for" "<" <Comma<Id>> ">")? = "for", "<", Comma<Id>, ">" => ActionFn(61);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action61::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("for" "<" <Comma<Id>> ">")? =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            4 => {
                // () =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_29(__nt), __end));
                2
            }
            5 => {
                // @L =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                3
            }
            6 => {
                // @R =  => ActionFn(22);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action22::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40R(__nt), __end));
                4
            }
            7 => {
                // Angle<Id> = "<", Comma<Id>, ">" => ActionFn(34);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            8 => {
                // Angle<Id> =  => ActionFn(64);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action64::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            9 => {
                // Angle<Ty> = "<", Comma<Ty>, ">" => ActionFn(30);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            10 => {
                // Angle<Ty> =  => ActionFn(65);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action65::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            11 => {
                // AssocTyName = "type", Id, ";" => ActionFn(10);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssocTyName(__nt), __end));
                7
            }
            12 => {
                // AssocTyName* =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            13 => {
                // AssocTyName* = AssocTyName+ => ActionFn(33);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            14 => {
                // AssocTyName+ = AssocTyName => ActionFn(53);
                let __sym0 = __pop_NtAssocTyName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            15 => {
                // AssocTyName+ = AssocTyName+, AssocTyName => ActionFn(54);
                let __sym1 = __pop_NtAssocTyName(__symbols);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            16 => {
                // AssocTyValue = "type", Id, "=", Ty, ";" => ActionFn(12);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtTy(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtAssocTyValue(__nt), __end));
                10
            }
            17 => {
                // AssocTyValue* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            18 => {
                // AssocTyValue* = AssocTyValue+ => ActionFn(29);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            19 => {
                // AssocTyValue+ = AssocTyValue => ActionFn(55);
                let __sym0 = __pop_NtAssocTyValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            20 => {
                // AssocTyValue+ = AssocTyValue+, AssocTyValue => ActionFn(56);
                let __sym1 = __pop_NtAssocTyValue(__symbols);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            21 => {
                // Comma1<Id> = Id => ActionFn(47);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            22 => {
                // Comma1<Id> = Comma<Id>, ",", Id => ActionFn(48);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            23 => {
                // Comma1<Id>? = Comma1<Id> => ActionFn(45);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            24 => {
                // Comma1<Id>? =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            25 => {
                // Comma1<Ty> = Ty => ActionFn(59);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            26 => {
                // Comma1<Ty> = Comma<Ty>, ",", Ty => ActionFn(60);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            27 => {
                // Comma1<Ty>? = Comma1<Ty> => ActionFn(57);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            28 => {
                // Comma1<Ty>? =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            29 => {
                // Comma1<WhereClause> = WhereClause => ActionFn(51);
                let __sym0 = __pop_NtWhereClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            30 => {
                // Comma1<WhereClause> = Comma<WhereClause>, ",", WhereClause => ActionFn(52);
                let __sym2 = __pop_NtWhereClause(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            31 => {
                // Comma1<WhereClause>? = Comma1<WhereClause> => ActionFn(49);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            32 => {
                // Comma1<WhereClause>? =  => ActionFn(50);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action50::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            33 => {
                // Comma<Id> = Comma1<Id> => ActionFn(73);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            34 => {
                // Comma<Id> =  => ActionFn(74);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action74::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            35 => {
                // Comma<Ty> = Comma1<Ty> => ActionFn(75);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            36 => {
                // Comma<Ty> =  => ActionFn(76);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action76::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            37 => {
                // Comma<WhereClause> = Comma1<WhereClause> => ActionFn(77);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            38 => {
                // Comma<WhereClause> =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            39 => {
                // Goals = "for", "<", Comma<Id>, ">", Comma<WhereClause> => ActionFn(62);
                let __sym4 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action62::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            40 => {
                // Goals = Comma<WhereClause> => ActionFn(63);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            41 => {
                // Id = r#"([:alpha:]|_)([:alnum:]|_)*"# => ActionFn(68);
                let __sym0 = __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                23
            }
            42 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", "}" => ActionFn(71);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            43 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", AssocTyValue+, "}" => ActionFn(72);
                let __sym9 = __pop_Term_22_7d_22(__symbols);
                let __sym8 = __pop_NtAssocTyValue_2b(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym9.2.clone();
                let __nt = super::__action72::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
                let __states_len = __states.len();
                __states.truncate(__states_len - 10);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            44 => {
                // Item = StructDefn => ActionFn(4);
                let __sym0 = __pop_NtStructDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            45 => {
                // Item = TraitDefn => ActionFn(5);
                let __sym0 = __pop_NtTraitDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            46 => {
                // Item = Impl => ActionFn(6);
                let __sym0 = __pop_NtImpl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            47 => {
                // Item* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            48 => {
                // Item* = Item+ => ActionFn(42);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            49 => {
                // Item+ = Item => ActionFn(43);
                let __sym0 = __pop_NtItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            50 => {
                // Item+ = Item+, Item => ActionFn(44);
                let __sym1 = __pop_NtItem(__symbols);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            51 => {
                // Program =  => ActionFn(79);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action79::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            52 => {
                // Program = Item+ => ActionFn(80);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            53 => {
                // ProjectionTy = "<", TraitRef<"as">, ">", "::", Id => ActionFn(16);
                let __sym4 = __pop_NtId(__symbols);
                let __sym3 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtTraitRef_3c_22as_22_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtProjectionTy(__nt), __end));
                29
            }
            54 => {
                // StructDefn = "struct", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(8);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtStructDefn(__nt), __end));
                30
            }
            55 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(69);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            56 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", AssocTyName+, "}" => ActionFn(70);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtAssocTyName_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action70::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            57 => {
                // TraitRef<":"> = Ty, ":", Id, Angle<Ty> => ActionFn(24);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22_3a_22_3e(__nt), __end));
                32
            }
            58 => {
                // TraitRef<"as"> = Ty, "as", Id, Angle<Ty> => ActionFn(26);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22as_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22as_22_3e(__nt), __end));
                33
            }
            59 => {
                // Ty = Id => ActionFn(13);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            60 => {
                // Ty = Id, "<", Comma<Ty>, ">" => ActionFn(14);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            61 => {
                // Ty = ProjectionTy => ActionFn(15);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            62 => {
                // WhereClause = TraitRef<":"> => ActionFn(19);
                let __sym0 = __pop_NtTraitRef_3c_22_3a_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            63 => {
                // WhereClause = ProjectionTy, "==", Ty => ActionFn(20);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_3d_3d_22(__symbols);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            64 => {
                // WhereClauses = "where", Comma<WhereClause> => ActionFn(17);
                let __sym1 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym0 = __pop_Term_22where_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            65 => {
                // WhereClauses =  => ActionFn(66);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action66::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            66 => {
                // __Goals = Goals => ActionFn(1);
                let __sym0 = __pop_NtGoals(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Goals(__nt), __end));
                37
            }
            67 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            68 => {
                // __Ty = Ty => ActionFn(2);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ty(__nt), __end));
                39
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22as_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22as_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22for_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22for_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22impl_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22impl_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22trait_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22trait_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22type_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22type_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22where_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22where_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40R<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40R(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AssocTyValue, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Ty>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<WhereClause>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGoals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGoals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtImpl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Impl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtImpl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Item, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProjectionTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProjectionTy, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProjectionTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStructDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStructDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22_3a_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22_3a_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22as_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22as_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClause<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WhereClause, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClause(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClauses<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClauses(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Goals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Goals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ty<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ty(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;

mod __parse__Ty {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22as_22(&'input str),
        Term_22for_22(&'input str),
        Term_22impl_22(&'input str),
        Term_22struct_22(&'input str),
        Term_22trait_22(&'input str),
        Term_22type_22(&'input str),
        Term_22where_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(Vec<Identifier>),
        Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(::std::option::Option<Vec<Identifier>>),
        Nt_28_29(()),
        Nt_40L(usize),
        Nt_40R(usize),
        NtAngle_3cId_3e(Vec<Identifier>),
        NtAngle_3cTy_3e(Vec<Ty>),
        NtAssocTyName(Identifier),
        NtAssocTyName_2a(::std::vec::Vec<Identifier>),
        NtAssocTyName_2b(::std::vec::Vec<Identifier>),
        NtAssocTyValue(AssocTyValue),
        NtAssocTyValue_2a(::std::vec::Vec<AssocTyValue>),
        NtAssocTyValue_2b(::std::vec::Vec<AssocTyValue>),
        NtComma1_3cId_3e(Vec<Identifier>),
        NtComma1_3cId_3e_3f(::std::option::Option<Vec<Identifier>>),
        NtComma1_3cTy_3e(Vec<Ty>),
        NtComma1_3cTy_3e_3f(::std::option::Option<Vec<Ty>>),
        NtComma1_3cWhereClause_3e(Vec<WhereClause>),
        NtComma1_3cWhereClause_3e_3f(::std::option::Option<Vec<WhereClause>>),
        NtComma_3cId_3e(Vec<Identifier>),
        NtComma_3cTy_3e(Vec<Ty>),
        NtComma_3cWhereClause_3e(Vec<WhereClause>),
        NtGoals(Goals),
        NtId(Identifier),
        NtImpl(Impl),
        NtItem(Item),
        NtItem_2a(::std::vec::Vec<Item>),
        NtItem_2b(::std::vec::Vec<Item>),
        NtProgram(Program),
        NtProjectionTy(ProjectionTy),
        NtStructDefn(StructDefn),
        NtTraitDefn(TraitDefn),
        NtTraitRef_3c_22_3a_22_3e(TraitRef),
        NtTraitRef_3c_22as_22_3e(TraitRef),
        NtTy(Ty),
        NtWhereClause(WhereClause),
        NtWhereClauses(Vec<WhereClause>),
        Nt____Goals(Goals),
        Nt____Program(Program),
        Nt____Ty(Ty),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0,
        // State 1
        0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 5
        0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -36, 0, 0, 0, 19, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
        // State 7
        0, 0, 0, 0, 21, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 12
        0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -35, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        25, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        -59, 0, 0, 0, 27, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -25, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 19
        -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -36, 0, 0, 0, 19, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
        // State 21
        0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        -36, 0, 0, 0, 19, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        25, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0,
        // State 30
        0, 0, 0, 0, 41, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -26, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        25, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        -36, 0, 0, 0, 19, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 42
        -60, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0,
        // State 44
        25, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -41, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -59,
        -61,
        -68,
        0,
        -41,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -60,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -53,
        -41,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 10, 11, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 24, 11, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 28, 11, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 29, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 35, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 45, 0, 0, 16, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_Ty<
        'input,
    >(
        input: &'input str,
    ) -> Result<Ty, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (17, _) if true => 17,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_3a_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3c_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_3d_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22as_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22for_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22impl_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22struct_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22trait_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22type_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22where_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Ty,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("for" "<" <Comma<Id>> ">") = "for", "<", Comma<Id>, ">" => ActionFn(39);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__nt), __end));
                0
            }
            2 => {
                // ("for" "<" <Comma<Id>> ">")? = "for", "<", Comma<Id>, ">" => ActionFn(61);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action61::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("for" "<" <Comma<Id>> ">")? =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__nt), __end));
                1
            }
            4 => {
                // () =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_29(__nt), __end));
                2
            }
            5 => {
                // @L =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                3
            }
            6 => {
                // @R =  => ActionFn(22);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action22::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40R(__nt), __end));
                4
            }
            7 => {
                // Angle<Id> = "<", Comma<Id>, ">" => ActionFn(34);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            8 => {
                // Angle<Id> =  => ActionFn(64);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action64::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cId_3e(__nt), __end));
                5
            }
            9 => {
                // Angle<Ty> = "<", Comma<Ty>, ">" => ActionFn(30);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            10 => {
                // Angle<Ty> =  => ActionFn(65);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action65::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAngle_3cTy_3e(__nt), __end));
                6
            }
            11 => {
                // AssocTyName = "type", Id, ";" => ActionFn(10);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssocTyName(__nt), __end));
                7
            }
            12 => {
                // AssocTyName* =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            13 => {
                // AssocTyName* = AssocTyName+ => ActionFn(33);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2a(__nt), __end));
                8
            }
            14 => {
                // AssocTyName+ = AssocTyName => ActionFn(53);
                let __sym0 = __pop_NtAssocTyName(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            15 => {
                // AssocTyName+ = AssocTyName+, AssocTyName => ActionFn(54);
                let __sym1 = __pop_NtAssocTyName(__symbols);
                let __sym0 = __pop_NtAssocTyName_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyName_2b(__nt), __end));
                9
            }
            16 => {
                // AssocTyValue = "type", Id, "=", Ty, ";" => ActionFn(12);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtTy(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22type_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtAssocTyValue(__nt), __end));
                10
            }
            17 => {
                // AssocTyValue* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            18 => {
                // AssocTyValue* = AssocTyValue+ => ActionFn(29);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2a(__nt), __end));
                11
            }
            19 => {
                // AssocTyValue+ = AssocTyValue => ActionFn(55);
                let __sym0 = __pop_NtAssocTyValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            20 => {
                // AssocTyValue+ = AssocTyValue+, AssocTyValue => ActionFn(56);
                let __sym1 = __pop_NtAssocTyValue(__symbols);
                let __sym0 = __pop_NtAssocTyValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAssocTyValue_2b(__nt), __end));
                12
            }
            21 => {
                // Comma1<Id> = Id => ActionFn(47);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            22 => {
                // Comma1<Id> = Comma<Id>, ",", Id => ActionFn(48);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e(__nt), __end));
                13
            }
            23 => {
                // Comma1<Id>? = Comma1<Id> => ActionFn(45);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            24 => {
                // Comma1<Id>? =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cId_3e_3f(__nt), __end));
                14
            }
            25 => {
                // Comma1<Ty> = Ty => ActionFn(59);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            26 => {
                // Comma1<Ty> = Comma<Ty>, ",", Ty => ActionFn(60);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                15
            }
            27 => {
                // Comma1<Ty>? = Comma1<Ty> => ActionFn(57);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            28 => {
                // Comma1<Ty>? =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e_3f(__nt), __end));
                16
            }
            29 => {
                // Comma1<WhereClause> = WhereClause => ActionFn(51);
                let __sym0 = __pop_NtWhereClause(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            30 => {
                // Comma1<WhereClause> = Comma<WhereClause>, ",", WhereClause => ActionFn(52);
                let __sym2 = __pop_NtWhereClause(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e(__nt), __end));
                17
            }
            31 => {
                // Comma1<WhereClause>? = Comma1<WhereClause> => ActionFn(49);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            32 => {
                // Comma1<WhereClause>? =  => ActionFn(50);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action50::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma1_3cWhereClause_3e_3f(__nt), __end));
                18
            }
            33 => {
                // Comma<Id> = Comma1<Id> => ActionFn(73);
                let __sym0 = __pop_NtComma1_3cId_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action73::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            34 => {
                // Comma<Id> =  => ActionFn(74);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action74::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cId_3e(__nt), __end));
                19
            }
            35 => {
                // Comma<Ty> = Comma1<Ty> => ActionFn(75);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action75::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            36 => {
                // Comma<Ty> =  => ActionFn(76);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action76::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                20
            }
            37 => {
                // Comma<WhereClause> = Comma1<WhereClause> => ActionFn(77);
                let __sym0 = __pop_NtComma1_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            38 => {
                // Comma<WhereClause> =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWhereClause_3e(__nt), __end));
                21
            }
            39 => {
                // Goals = "for", "<", Comma<Id>, ">", Comma<WhereClause> => ActionFn(62);
                let __sym4 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cId_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_Term_22for_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action62::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            40 => {
                // Goals = Comma<WhereClause> => ActionFn(63);
                let __sym0 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGoals(__nt), __end));
                22
            }
            41 => {
                // Id = r#"([:alpha:]|_)([:alnum:]|_)*"# => ActionFn(68);
                let __sym0 = __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action68::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                23
            }
            42 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", "}" => ActionFn(71);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action71::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            43 => {
                // Impl = "impl", Angle<Id>, Id, Angle<Ty>, "for", Ty, WhereClauses, "{", AssocTyValue+, "}" => ActionFn(72);
                let __sym9 = __pop_Term_22_7d_22(__symbols);
                let __sym8 = __pop_NtAssocTyValue_2b(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_NtWhereClauses(__symbols);
                let __sym5 = __pop_NtTy(__symbols);
                let __sym4 = __pop_Term_22for_22(__symbols);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym9.2.clone();
                let __nt = super::__action72::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
                let __states_len = __states.len();
                __states.truncate(__states_len - 10);
                __symbols.push((__start, __Symbol::NtImpl(__nt), __end));
                24
            }
            44 => {
                // Item = StructDefn => ActionFn(4);
                let __sym0 = __pop_NtStructDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            45 => {
                // Item = TraitDefn => ActionFn(5);
                let __sym0 = __pop_NtTraitDefn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            46 => {
                // Item = Impl => ActionFn(6);
                let __sym0 = __pop_NtImpl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem(__nt), __end));
                25
            }
            47 => {
                // Item* =  => ActionFn(41);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action41::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            48 => {
                // Item* = Item+ => ActionFn(42);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2a(__nt), __end));
                26
            }
            49 => {
                // Item+ = Item => ActionFn(43);
                let __sym0 = __pop_NtItem(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            50 => {
                // Item+ = Item+, Item => ActionFn(44);
                let __sym1 = __pop_NtItem(__symbols);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtItem_2b(__nt), __end));
                27
            }
            51 => {
                // Program =  => ActionFn(79);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action79::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            52 => {
                // Program = Item+ => ActionFn(80);
                let __sym0 = __pop_NtItem_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action80::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                28
            }
            53 => {
                // ProjectionTy = "<", TraitRef<"as">, ">", "::", Id => ActionFn(16);
                let __sym4 = __pop_NtId(__symbols);
                let __sym3 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_NtTraitRef_3c_22as_22_3e(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtProjectionTy(__nt), __end));
                29
            }
            54 => {
                // StructDefn = "struct", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(8);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtStructDefn(__nt), __end));
                30
            }
            55 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", "}" => ActionFn(69);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action69::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            56 => {
                // TraitDefn = "trait", Id, Angle<Id>, WhereClauses, "{", AssocTyName+, "}" => ActionFn(70);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtAssocTyName_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtWhereClauses(__symbols);
                let __sym2 = __pop_NtAngle_3cId_3e(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22trait_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action70::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtTraitDefn(__nt), __end));
                31
            }
            57 => {
                // TraitRef<":"> = Ty, ":", Id, Angle<Ty> => ActionFn(24);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22_3a_22_3e(__nt), __end));
                32
            }
            58 => {
                // TraitRef<"as"> = Ty, "as", Id, Angle<Ty> => ActionFn(26);
                let __sym3 = __pop_NtAngle_3cTy_3e(__symbols);
                let __sym2 = __pop_NtId(__symbols);
                let __sym1 = __pop_Term_22as_22(__symbols);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTraitRef_3c_22as_22_3e(__nt), __end));
                33
            }
            59 => {
                // Ty = Id => ActionFn(13);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            60 => {
                // Ty = Id, "<", Comma<Ty>, ">" => ActionFn(14);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            61 => {
                // Ty = ProjectionTy => ActionFn(15);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                34
            }
            62 => {
                // WhereClause = TraitRef<":"> => ActionFn(19);
                let __sym0 = __pop_NtTraitRef_3c_22_3a_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            63 => {
                // WhereClause = ProjectionTy, "==", Ty => ActionFn(20);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_3d_3d_22(__symbols);
                let __sym0 = __pop_NtProjectionTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWhereClause(__nt), __end));
                35
            }
            64 => {
                // WhereClauses = "where", Comma<WhereClause> => ActionFn(17);
                let __sym1 = __pop_NtComma_3cWhereClause_3e(__symbols);
                let __sym0 = __pop_Term_22where_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            65 => {
                // WhereClauses =  => ActionFn(66);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action66::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWhereClauses(__nt), __end));
                36
            }
            66 => {
                // __Goals = Goals => ActionFn(1);
                let __sym0 = __pop_NtGoals(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Goals(__nt), __end));
                37
            }
            67 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                38
            }
            68 => {
                // __Ty = Ty => ActionFn(2);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22as_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22as_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22for_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22for_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22impl_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22impl_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22trait_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22trait_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22type_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22type_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22where_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22where_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_3aalpha_3a_5d_7c___29_28_5b_3aalnum_3a_5d_7c___29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22for_22_20_22_3c_22_20_3cComma_3cId_3e_3e_20_22_3e_22_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40R<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40R(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAngle_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAngle_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyName_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyName_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AssocTyValue, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssocTyValue_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AssocTyValue>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssocTyValue_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cId_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Identifier>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cId_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<Ty>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cWhereClause_3e_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Vec<WhereClause>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cWhereClause_3e_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cId_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cId_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Ty>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWhereClause_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWhereClause_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGoals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGoals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtImpl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Impl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtImpl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Item, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtItem_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Item>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtItem_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProjectionTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ProjectionTy, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProjectionTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStructDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StructDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStructDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitDefn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitDefn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitDefn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22_3a_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22_3a_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTraitRef_3c_22as_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TraitRef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTraitRef_3c_22as_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClause<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WhereClause, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClause(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWhereClauses<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WhereClause>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWhereClauses(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Goals<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Goals, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Goals(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ty<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ty, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ty(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Ty::parse_Ty;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        44 => /* ',' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        98 ... 101 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        103 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        106 ... 114 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        117 ... 118 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        120 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        58 => /* ':' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        61 => /* '=' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        115 ... 120 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        121 => /* 'y' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        122 => /* 'z' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 103 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        104 => /* 'h' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        105 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        106 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((17, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Goals, usize),
) -> Goals
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ty, usize),
) -> Ty
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Item>, usize),
) -> Program
{
    Program { items: __0 }
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, StructDefn, usize),
) -> Item
{
    Item::StructDefn(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TraitDefn, usize),
) -> Item
{
    Item::TraitDefn(__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Impl, usize),
) -> Item
{
    Item::Impl(__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, ::std::option::Option<Vec<Identifier>>, usize),
    (_, w, _): (usize, Vec<WhereClause>, usize),
) -> Goals
{
    Goals {
        parameters: p.unwrap_or(vec![]),
        where_clauses: w,
    }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Identifier, usize),
    (_, p, _): (usize, Vec<Identifier>, usize),
    (_, w, _): (usize, Vec<WhereClause>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> StructDefn
{
    StructDefn {
        name: n,
        parameters: p,
        where_clauses: w
    }
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Identifier, usize),
    (_, p, _): (usize, Vec<Identifier>, usize),
    (_, w, _): (usize, Vec<WhereClause>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TraitDefn
{
    TraitDefn {
        name: n,
        parameters: p,
        where_clauses: w,
        assoc_ty_names: a,
    }
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Identifier
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Vec<Identifier>, usize),
    (_, t, _): (usize, Identifier, usize),
    (_, a, _): (usize, Vec<Ty>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, Ty, usize),
    (_, w, _): (usize, Vec<WhereClause>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, assoc, _): (usize, ::std::vec::Vec<AssocTyValue>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Impl
{
    {
        let mut args = vec![s];
        args.extend(a);
        Impl {
            parameters: p,
            trait_ref: TraitRef { trait_name: t, args: args },
            where_clauses: w,
            assoc_ty_values: assoc,
        }
    }
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Ty, usize),
    (_, _, _): (usize, &'input str, usize),
) -> AssocTyValue
{
    AssocTyValue {
        name: n,
        value: v,
    }
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Identifier, usize),
) -> Ty
{
    Ty::Id { name: n}
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<Ty>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Ty
{
    Ty::Apply { name: n, args: a }
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, ProjectionTy, usize),
) -> Ty
{
    Ty::Projection { proj: p }
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, TraitRef, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Identifier, usize),
) -> ProjectionTy
{
    ProjectionTy {
        trait_ref: t, name: n
    }
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<WhereClause>, usize),
) -> Vec<WhereClause>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> Vec<WhereClause>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, TraitRef, usize),
) -> WhereClause
{
    WhereClause::Implemented { trait_ref: t }
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, ProjectionTy, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Ty, usize),
) -> WhereClause
{
    WhereClause::ProjectionEq { projection: p, ty: t }
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, s, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> Identifier
{
    Identifier {
        str: intern(s),
        span: Span::new(l, r),
    }
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, Ty, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Identifier, usize),
    (_, a, _): (usize, Vec<Ty>, usize),
) -> TraitRef
{
    {
        let mut args = vec![s];
        args.extend(a);
        TraitRef {
            trait_name: t,
            args: args,
        }
    }
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ()
{
    ()
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, Ty, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Identifier, usize),
    (_, a, _): (usize, Vec<Ty>, usize),
) -> TraitRef
{
    {
        let mut args = vec![s];
        args.extend(a);
        TraitRef {
            trait_name: t,
            args: args,
        }
    }
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::option::Option<Vec<Ty>>, usize),
) -> Vec<Ty>
{
    __0.unwrap_or(vec![])
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<AssocTyValue>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<AssocTyValue>, usize),
) -> ::std::vec::Vec<AssocTyValue>
{
    v
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Ty>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Ty>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> Vec<Ty>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Identifier>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> ::std::vec::Vec<Identifier>
{
    v
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Identifier>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Identifier>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> Vec<Identifier>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::option::Option<Vec<WhereClause>>, usize),
) -> Vec<WhereClause>
{
    __0.unwrap_or(vec![])
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Identifier>, usize),
) -> ::std::option::Option<Vec<Identifier>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<Identifier>>
{
    None
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Identifier>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Identifier>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::option::Option<Vec<Identifier>>, usize),
) -> Vec<Identifier>
{
    __0.unwrap_or(vec![])
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Item>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Item>, usize),
) -> ::std::vec::Vec<Item>
{
    v
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Item, usize),
) -> ::std::vec::Vec<Item>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Item>, usize),
    (_, e, _): (usize, Item, usize),
) -> ::std::vec::Vec<Item>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Identifier>, usize),
) -> ::std::option::Option<Vec<Identifier>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<Identifier>>
{
    None
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Identifier, usize),
) -> Vec<Identifier>
{
    vec![t]
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<Identifier>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Identifier, usize),
) -> Vec<Identifier>
{
    {
        let mut v = v;
        v.push(t);
        v
    }
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<WhereClause>, usize),
) -> ::std::option::Option<Vec<WhereClause>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<WhereClause>>
{
    None
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, WhereClause, usize),
) -> Vec<WhereClause>
{
    vec![t]
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<WhereClause>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, WhereClause, usize),
) -> Vec<WhereClause>
{
    {
        let mut v = v;
        v.push(t);
        v
    }
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, e, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, AssocTyValue, usize),
) -> ::std::vec::Vec<AssocTyValue>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<AssocTyValue>, usize),
    (_, e, _): (usize, AssocTyValue, usize),
) -> ::std::vec::Vec<AssocTyValue>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Ty>, usize),
) -> ::std::option::Option<Vec<Ty>>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<Ty>>
{
    None
}

#[allow(unused_variables)]
pub fn __action59<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Ty, usize),
) -> Vec<Ty>
{
    vec![t]
}

#[allow(unused_variables)]
pub fn __action60<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<Ty>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Ty, usize),
) -> Vec<Ty>
{
    {
        let mut v = v;
        v.push(t);
        v
    }
}

#[allow(unused_variables)]
pub fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Identifier>, usize),
    __3: (usize, &'input str, usize),
) -> ::std::option::Option<Vec<Identifier>>
{
    let __start0 = __0.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action39(
        input,
        __0,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Identifier>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Vec<WhereClause>, usize),
) -> Goals
{
    let __start0 = __0.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action61(
        input,
        __0,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<WhereClause>, usize),
) -> Goals
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action64<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Identifier>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action65<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ty>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action66<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<WhereClause>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action67<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Identifier
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action68<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Identifier
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action22(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action69<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Vec<Identifier>, usize),
    __3: (usize, Vec<WhereClause>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> TraitDefn
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action32(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action70<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Vec<Identifier>, usize),
    __3: (usize, Vec<WhereClause>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Identifier>, usize),
    __6: (usize, &'input str, usize),
) -> TraitDefn
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action33(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action71<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<Identifier>, usize),
    __2: (usize, Identifier, usize),
    __3: (usize, Vec<Ty>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, Ty, usize),
    __6: (usize, Vec<WhereClause>, usize),
    __7: (usize, &'input str, usize),
    __8: (usize, &'input str, usize),
) -> Impl
{
    let __start0 = __7.2.clone();
    let __end0 = __8.0.clone();
    let __temp0 = __action28(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
        __8,
    )
}

#[allow(unused_variables)]
pub fn __action72<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<Identifier>, usize),
    __2: (usize, Identifier, usize),
    __3: (usize, Vec<Ty>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, Ty, usize),
    __6: (usize, Vec<WhereClause>, usize),
    __7: (usize, &'input str, usize),
    __8: (usize, ::std::vec::Vec<AssocTyValue>, usize),
    __9: (usize, &'input str, usize),
) -> Impl
{
    let __start0 = __8.0.clone();
    let __end0 = __8.2.clone();
    let __temp0 = __action29(
        input,
        __8,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
        __temp0,
        __9,
    )
}

#[allow(unused_variables)]
pub fn __action73<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<Identifier>, usize),
) -> Vec<Identifier>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action45(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action74<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Identifier>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action46(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action75<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<Ty>, usize),
) -> Vec<Ty>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action57(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action76<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Ty>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action58(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action77<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<WhereClause>, usize),
) -> Vec<WhereClause>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action78<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<WhereClause>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action79<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action41(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Item>, usize),
) -> Program
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
