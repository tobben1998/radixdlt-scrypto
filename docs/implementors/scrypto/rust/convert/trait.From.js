(function() {var implementors = {};
implementors["scrypto"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"scrypto/rust/convert/trait.Into.html\" title=\"trait scrypto::rust::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"scrypto/core/struct.Package.html\" title=\"struct scrypto::core::Package\">Package</a>&gt;, S:&nbsp;<a class=\"trait\" href=\"scrypto/rust/convert/trait.AsRef.html\" title=\"trait scrypto::rust::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">(</a>A, S<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"scrypto/core/struct.Blueprint.html\" title=\"struct scrypto::core::Blueprint\">Blueprint</a>","synthetic":false,"types":["scrypto::core::blueprint::Blueprint"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/core/struct.Blueprint.html\" title=\"struct scrypto::core::Blueprint\">Blueprint</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">(</a><a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>, <a class=\"struct\" href=\"scrypto/rust/string/struct.String.html\" title=\"struct scrypto::rust::string::String\">String</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">)</a>","synthetic":false,"types":["scrypto::types::address::Address","alloc::string::String"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>&gt; for <a class=\"struct\" href=\"scrypto/core/struct.Component.html\" title=\"struct scrypto::core::Component\">Component</a>","synthetic":false,"types":["scrypto::core::component::Component"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/core/struct.Component.html\" title=\"struct scrypto::core::Component\">Component</a>&gt; for <a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>","synthetic":false,"types":["scrypto::types::address::Address"]},{"text":"impl&lt;K:&nbsp;<a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a> + <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>, V:&nbsp;<a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a> + <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>&gt; <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/types/struct.Mid.html\" title=\"struct scrypto::types::Mid\">Mid</a>&gt; for <a class=\"struct\" href=\"scrypto/core/struct.LazyMap.html\" title=\"struct scrypto::core::LazyMap\">LazyMap</a>&lt;K, V&gt;","synthetic":false,"types":["scrypto::core::lazy_map::LazyMap"]},{"text":"impl&lt;K:&nbsp;<a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a> + <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>, V:&nbsp;<a class=\"trait\" href=\"sbor/encode/trait.Encode.html\" title=\"trait sbor::encode::Encode\">Encode</a> + <a class=\"trait\" href=\"sbor/decode/trait.Decode.html\" title=\"trait sbor::decode::Decode\">Decode</a>&gt; <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/core/struct.LazyMap.html\" title=\"struct scrypto::core::LazyMap\">LazyMap</a>&lt;K, V&gt;&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Mid.html\" title=\"struct scrypto::types::Mid\">Mid</a>","synthetic":false,"types":["scrypto::types::mid::Mid"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>&gt; for <a class=\"struct\" href=\"scrypto/core/struct.Package.html\" title=\"struct scrypto::core::Package\">Package</a>","synthetic":false,"types":["scrypto::core::package::Package"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/core/struct.Package.html\" title=\"struct scrypto::core::Package\">Package</a>&gt; for <a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>","synthetic":false,"types":["scrypto::types::address::Address"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/types/struct.Bid.html\" title=\"struct scrypto::types::Bid\">Bid</a>&gt; for <a class=\"struct\" href=\"scrypto/resource/struct.Bucket.html\" title=\"struct scrypto::resource::Bucket\">Bucket</a>","synthetic":false,"types":["scrypto::resource::bucket::Bucket"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/resource/struct.Bucket.html\" title=\"struct scrypto::resource::Bucket\">Bucket</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Bid.html\" title=\"struct scrypto::types::Bid\">Bid</a>","synthetic":false,"types":["scrypto::types::bid::Bid"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/types/struct.Rid.html\" title=\"struct scrypto::types::Rid\">Rid</a>&gt; for <a class=\"struct\" href=\"scrypto/resource/struct.BucketRef.html\" title=\"struct scrypto::resource::BucketRef\">BucketRef</a>","synthetic":false,"types":["scrypto::resource::bucket_ref::BucketRef"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/resource/struct.BucketRef.html\" title=\"struct scrypto::resource::BucketRef\">BucketRef</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Rid.html\" title=\"struct scrypto::types::Rid\">Rid</a>","synthetic":false,"types":["scrypto::types::rid::Rid"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"scrypto/resource/trait.NonFungibleData.html\" title=\"trait scrypto::resource::NonFungibleData\">NonFungibleData</a>&gt; <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">(</a><a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>, <a class=\"struct\" href=\"scrypto/types/struct.NonFungibleKey.html\" title=\"struct scrypto::types::NonFungibleKey\">NonFungibleKey</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"scrypto/resource/struct.NonFungible.html\" title=\"struct scrypto::resource::NonFungible\">NonFungible</a>&lt;T&gt;","synthetic":false,"types":["scrypto::resource::non_fungible::NonFungible"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>&gt; for <a class=\"struct\" href=\"scrypto/resource/struct.ResourceDef.html\" title=\"struct scrypto::resource::ResourceDef\">ResourceDef</a>","synthetic":false,"types":["scrypto::resource::resource_def::ResourceDef"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/resource/struct.ResourceDef.html\" title=\"struct scrypto::resource::ResourceDef\">ResourceDef</a>&gt; for <a class=\"enum\" href=\"scrypto/types/enum.Address.html\" title=\"enum scrypto::types::Address\">Address</a>","synthetic":false,"types":["scrypto::types::address::Address"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/types/struct.Vid.html\" title=\"struct scrypto::types::Vid\">Vid</a>&gt; for <a class=\"struct\" href=\"scrypto/resource/struct.Vault.html\" title=\"struct scrypto::resource::Vault\">Vault</a>","synthetic":false,"types":["scrypto::resource::vault::Vault"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/resource/struct.Vault.html\" title=\"struct scrypto::resource::Vault\">Vault</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Vid.html\" title=\"struct scrypto::types::Vid\">Vid</a>","synthetic":false,"types":["scrypto::types::vid::Vid"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i8.html\">i8</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i16.html\">i16</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i32.html\">i32</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i64.html\">i64</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i128.html\">i128</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.isize.html\">isize</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;&amp;'_ <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/rust/string/struct.String.html\" title=\"struct scrypto::rust::string::String\">String</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.bool.html\">bool</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.BigDecimal.html\" title=\"struct scrypto::types::BigDecimal\">BigDecimal</a>","synthetic":false,"types":["scrypto::types::big_decimal::BigDecimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i8.html\">i8</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i16.html\">i16</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i32.html\">i32</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i64.html\">i64</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.i128.html\">i128</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.isize.html\">isize</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;&amp;'_ <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/rust/string/struct.String.html\" title=\"struct scrypto::rust::string::String\">String</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.bool.html\">bool</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.Decimal.html\" title=\"struct scrypto::types::Decimal\">Decimal</a>","synthetic":false,"types":["scrypto::types::decimal::Decimal"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"struct\" href=\"scrypto/types/struct.H256.html\" title=\"struct scrypto::types::H256\">H256</a>&gt; for <a class=\"struct\" href=\"scrypto/rust/vec/struct.Vec.html\" title=\"struct scrypto::rust::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u8.html\">u8</a>&gt;","synthetic":false,"types":["alloc::vec::Vec"]},{"text":"impl <a class=\"trait\" href=\"scrypto/rust/convert/trait.From.html\" title=\"trait scrypto::rust::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"scrypto/types/struct.NonFungibleKey.html\" title=\"struct scrypto::types::NonFungibleKey\">NonFungibleKey</a>","synthetic":false,"types":["scrypto::types::non_fungible_key::NonFungibleKey"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()