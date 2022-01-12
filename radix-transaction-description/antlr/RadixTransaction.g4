grammar RadixTransaction;

transaction: instruction+;

instruction:
	'DECLARE_TEMP_BUCKET' string ';'
	| 'DECLARE_TEMP_BUCKET_REF' string ';'
	| 'TAKE_FROM_CONTEXT' decimal address bucket ';'
	| 'BORROW_FROM_CONTEXT' decimal address bucketRef  ';'
	| 'CALL_FUNCTION' address string string value* ';'
	| 'CALL_METHOD' address string value*  ';'
	| 'DROP_ALL_BUCKET_REFS' ';'
	| 'DEPOSIT_ALL_BUCKETS' address ';'
	;

value:
	unit
	| bool
	| i8
	| i16
	| i32
	| i64
	| i128
	| u8
	| u16
	| u32
	| u64
	| u128
	| string
	| struct
	| enumeration
	| option
	| box
	| array
	| tuple
	| result
	| vec
	| treeSet
	| hashSet
	| treeMap
	| hashMap
	| decimal
	| bigDecimal
	| address
	| hash
	| bucket
	| bucketRef
	| map
	| vault;

unit: 'unit';

bool: 'true' | 'false';

i8: I8;
i16: I16;
i32: I32;
i64: I64;
i128: I128;

u8: U8;
u16: U16;
u32: U32;
u64: U64;
u128: U128;

string: STRING;

struct: 'struct' '(' values? ')';

enumeration: 'enum' '(' u8 ')';

option: 'some' '(' value ')' | 'none';

box: 'box' '(' value ')';

array: '[' values? ']';

tuple: '(' values? ')';

result: 'ok' '(' value ')' | 'err' '(' value ')';

vec: 'vec' '(' values? ')';

treeSet: 'tree_set' '(' values? ')';

treeMap: 'tree_map' '(' values? ')';

hashSet: 'hash_set' '(' values? ')';

hashMap: 'hash_map' '(' values? ')';

values: value | value ',' values;

decimal: 'decimal' '(' DECIMAL ')';

bigDecimal: 'big_decimal' '(' DECIMAL ')';

address: 'address' '(' string ')';

hash: 'hash' '(' string ')';

bucket: 'bucket' '(' (string | u32) ')';

bucketRef: 'bucket_ref' '(' (string | u32) ')';

map: 'map' '(' string ')';

vault: 'vault' '(' string ')';

// String literals
STRING: '"' (ESC | SAFECODEPOINT)* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];
fragment SAFECODEPOINT: ~ ["\\\u0000-\u001F];

// Numbers
I8: '-'? INT 'i8';
I16: '-'? INT 'i16';
I32: '-'? INT 'i32';
I64: '-'? INT 'i64';
I128: '-'? INT 'i128';
U8: INT 'u8';
U16: INT 'u16';
U32: INT 'u32';
U64: INT 'u64';
U128: INT 'u128';
DECIMAL: '-'? INT ('.' [0-9]+)?;
fragment INT: '0' | [1-9] [0-9]*;

// Whitespace
WHITESPACE: [ \t\n\r]+ -> skip;