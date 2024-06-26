#define CACHE                                    0
#define POP_TOP                                  1
#define PUSH_NULL                                2
#define NOP                                      9
#define UNARY_POSITIVE                          10
#define UNARY_NEGATIVE                          11
#define UNARY_NOT                               12
#define UNARY_INVERT                            15
#define BINARY_SUBSCR                           25
#define GET_LEN                                 30
#define MATCH_MAPPING                           31
#define MATCH_SEQUENCE                          32
#define MATCH_KEYS                              33
#define PUSH_EXC_INFO                           35
#define CHECK_EXC_MATCH                         36
#define CHECK_EG_MATCH                          37
#define WITH_EXCEPT_START                       49
#define GET_AITER                               50
#define GET_ANEXT                               51
#define BEFORE_ASYNC_WITH                       52
#define BEFORE_WITH                             53
#define END_ASYNC_FOR                           54
#define STORE_SUBSCR                            60
#define DELETE_SUBSCR                           61
#define GET_ITER                                68
#define GET_YIELD_FROM_ITER                     69
#define PRINT_EXPR                              70
#define LOAD_BUILD_CLASS                        71
#define LOAD_ASSERTION_ERROR                    74
#define RETURN_GENERATOR                        75
#define LIST_TO_TUPLE                           82
#define RETURN_VALUE                            83
#define IMPORT_STAR                             84
#define SETUP_ANNOTATIONS                       85
#define YIELD_VALUE                             86
#define ASYNC_GEN_WRAP                          87
#define PREP_RERAISE_STAR                       88
#define POP_EXCEPT                              89
#define HAVE_ARGUMENT                           90
#define STORE_NAME                              90
#define DELETE_NAME                             91
#define UNPACK_SEQUENCE                         92
#define FOR_ITER                                93
#define UNPACK_EX                               94
#define STORE_ATTR                              95
#define DELETE_ATTR                             96
#define STORE_GLOBAL                            97
#define DELETE_GLOBAL                           98
#define SWAP                                    99
#define LOAD_CONST                             100
#define LOAD_NAME                              101
#define BUILD_TUPLE                            102
#define BUILD_LIST                             103
#define BUILD_SET                              104
#define BUILD_MAP                              105
#define LOAD_ATTR                              106
#define COMPARE_OP                             107
#define IMPORT_NAME                            108
#define IMPORT_FROM                            109
#define JUMP_FORWARD                           110
#define JUMP_IF_FALSE_OR_POP                   111
#define JUMP_IF_TRUE_OR_POP                    112
#define POP_JUMP_FORWARD_IF_FALSE              114
#define POP_JUMP_FORWARD_IF_TRUE               115
#define LOAD_GLOBAL                            116
#define IS_OP                                  117
#define CONTAINS_OP                            118
#define RERAISE                                119
#define COPY                                   120
#define BINARY_OP                              122
#define SEND                                   123
#define LOAD_FAST                              124
#define STORE_FAST                             125
#define DELETE_FAST                            126
#define POP_JUMP_FORWARD_IF_NOT_NONE           128
#define POP_JUMP_FORWARD_IF_NONE               129
#define RAISE_VARARGS                          130
#define GET_AWAITABLE                          131
#define MAKE_FUNCTION                          132
#define BUILD_SLICE                            133
#define JUMP_BACKWARD_NO_INTERRUPT             134
#define MAKE_CELL                              135
#define LOAD_CLOSURE                           136
#define LOAD_DEREF                             137
#define STORE_DEREF                            138
#define DELETE_DEREF                           139
#define JUMP_BACKWARD                          140
#define CALL_FUNCTION_EX                       142
#define EXTENDED_ARG                           144
#define LIST_APPEND                            145
#define SET_ADD                                146
#define MAP_ADD                                147
#define LOAD_CLASSDEREF                        148
#define COPY_FREE_VARS                         149
#define RESUME                                 151
#define MATCH_CLASS                            152
#define FORMAT_VALUE                           155
#define BUILD_CONST_KEY_MAP                    156
#define BUILD_STRING                           157
#define LOAD_METHOD                            160
#define LIST_EXTEND                            162
#define SET_UPDATE                             163
#define DICT_MERGE                             164
#define DICT_UPDATE                            165
#define PRECALL                                166
#define CALL                                   171
#define KW_NAMES                               172
#define POP_JUMP_BACKWARD_IF_NOT_NONE          173
#define POP_JUMP_BACKWARD_IF_NONE              174
#define POP_JUMP_BACKWARD_IF_FALSE             175
#define POP_JUMP_BACKWARD_IF_TRUE              176
#define BINARY_OP_ADAPTIVE                       3
#define BINARY_OP_ADD_FLOAT                      4
#define BINARY_OP_ADD_INT                        5
#define BINARY_OP_ADD_UNICODE                    6
#define BINARY_OP_INPLACE_ADD_UNICODE            7
#define BINARY_OP_MULTIPLY_FLOAT                 8
#define BINARY_OP_MULTIPLY_INT                  13
#define BINARY_OP_SUBTRACT_FLOAT                14
#define BINARY_OP_SUBTRACT_INT                  16
#define BINARY_SUBSCR_ADAPTIVE                  17
#define BINARY_SUBSCR_DICT                      18
#define BINARY_SUBSCR_GETITEM                   19
#define BINARY_SUBSCR_LIST_INT                  20
#define BINARY_SUBSCR_TUPLE_INT                 21
#define CALL_ADAPTIVE                           22
#define CALL_PY_EXACT_ARGS                      23
#define CALL_PY_WITH_DEFAULTS                   24
#define COMPARE_OP_ADAPTIVE                     26
#define COMPARE_OP_FLOAT_JUMP                   27
#define COMPARE_OP_INT_JUMP                     28
#define COMPARE_OP_STR_JUMP                     29
#define EXTENDED_ARG_QUICK                      34
#define JUMP_BACKWARD_QUICK                     38
#define LOAD_ATTR_ADAPTIVE                      39
#define LOAD_ATTR_INSTANCE_VALUE                40
#define LOAD_ATTR_MODULE                        41
#define LOAD_ATTR_SLOT                          42
#define LOAD_ATTR_WITH_HINT                     43
#define LOAD_CONST__LOAD_FAST                   44
#define LOAD_FAST__LOAD_CONST                   45
#define LOAD_FAST__LOAD_FAST                    46
#define LOAD_GLOBAL_ADAPTIVE                    47
#define LOAD_GLOBAL_BUILTIN                     48
#define LOAD_GLOBAL_MODULE                      55
#define LOAD_METHOD_ADAPTIVE                    56
#define LOAD_METHOD_CLASS                       57
#define LOAD_METHOD_MODULE                      58
#define LOAD_METHOD_NO_DICT                     59
#define LOAD_METHOD_WITH_DICT                   62
#define LOAD_METHOD_WITH_VALUES                 63
#define PRECALL_ADAPTIVE                        64
#define PRECALL_BOUND_METHOD                    65
#define PRECALL_BUILTIN_CLASS                   66
#define PRECALL_BUILTIN_FAST_WITH_KEYWORDS      67
#define PRECALL_METHOD_DESCRIPTOR_FAST_WITH_KEYWORDS  72
#define PRECALL_NO_KW_BUILTIN_FAST              73
#define PRECALL_NO_KW_BUILTIN_O                 76
#define PRECALL_NO_KW_ISINSTANCE                77
#define PRECALL_NO_KW_LEN                       78
#define PRECALL_NO_KW_LIST_APPEND               79
#define PRECALL_NO_KW_METHOD_DESCRIPTOR_FAST    80
#define PRECALL_NO_KW_METHOD_DESCRIPTOR_NOARGS  81
#define PRECALL_NO_KW_METHOD_DESCRIPTOR_O      113
#define PRECALL_NO_KW_STR_1                    121
#define PRECALL_NO_KW_TUPLE_1                  127
#define PRECALL_NO_KW_TYPE_1                   141
#define PRECALL_PYFUNC                         143
#define RESUME_QUICK                           150
#define STORE_ATTR_ADAPTIVE                    153
#define STORE_ATTR_INSTANCE_VALUE              154
#define STORE_ATTR_SLOT                        158
#define STORE_ATTR_WITH_HINT                   159
#define STORE_FAST__LOAD_FAST                  161
#define STORE_FAST__STORE_FAST                 167
#define STORE_SUBSCR_ADAPTIVE                  168
#define STORE_SUBSCR_DICT                      169
#define STORE_SUBSCR_LIST_INT                  170
#define UNPACK_SEQUENCE_ADAPTIVE               177
#define UNPACK_SEQUENCE_LIST                   178
#define UNPACK_SEQUENCE_TUPLE                  179
#define UNPACK_SEQUENCE_TWO_TUPLE              180
#define DO_TRACING                             255
