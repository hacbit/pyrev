(function() {var type_impls = {
"pyrev":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-OrderMap%3CK,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#91\">source</a><a href=\"#impl-Clone-for-OrderMap%3CK,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pyrev/core/common/struct.OrderMap.html\" title=\"struct pyrev::core::common::OrderMap\">OrderMap</a>&lt;K, V&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#91\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"pyrev/core/common/struct.OrderMap.html\" title=\"struct pyrev::core::common::OrderMap\">OrderMap</a>&lt;K, V&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","pyrev::core::parse_opcode::CodeObjectMap"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-OrderMap%3CK,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#91\">source</a><a href=\"#impl-Debug-for-OrderMap%3CK,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"pyrev/core/common/struct.OrderMap.html\" title=\"struct pyrev::core::common::OrderMap\">OrderMap</a>&lt;K, V&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#91\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","pyrev::core::parse_opcode::CodeObjectMap"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decompiler-for-OrderMap%3CString,+Vec%3COpcodeInstruction%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/pyrev/core/decompile.rs.html#13-54\">source</a><a href=\"#impl-Decompiler-for-OrderMap%3CString,+Vec%3COpcodeInstruction%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"pyrev/core/decompile/trait.Decompiler.html\" title=\"trait pyrev::core::decompile::Decompiler\">Decompiler</a> for <a class=\"struct\" href=\"pyrev/core/common/struct.OrderMap.html\" title=\"struct pyrev::core::common::OrderMap\">OrderMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"pyrev/core/opcode/struct.OpcodeInstruction.html\" title=\"struct pyrev::core::opcode::OpcodeInstruction\">OpcodeInstruction</a>&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decompile\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pyrev/core/decompile.rs.html#15-49\">source</a><a href=\"#method.decompile\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"pyrev/core/decompile/trait.Decompiler.html#tymethod.decompile\" class=\"fn\">decompile</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"pyrev/core/decompile/struct.DecompiledCode.html\" title=\"struct pyrev::core::decompile::DecompiledCode\">DecompiledCode</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a>&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>从字节码对象映射表中解析为AST, 然后再从AST解析为代码</p>\n</div></details><section id=\"method.optimize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/pyrev/core/decompile.rs.html#51-53\">source</a><a href=\"#method.optimize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"pyrev/core/decompile/trait.Decompiler.html#tymethod.optimize\" class=\"fn\">optimize</a>(&amp;self, expr: &amp;Expr) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Expr, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a>&gt;&gt;</h4></section></div></details>","Decompiler","pyrev::core::parse_opcode::CodeObjectMap"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OrderMap%3CK,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#99-157\">source</a><a href=\"#impl-OrderMap%3CK,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K, V&gt; <a class=\"struct\" href=\"pyrev/core/common/struct.OrderMap.html\" title=\"struct pyrev::core::common::OrderMap\">OrderMap</a>&lt;K, V&gt;<div class=\"where\">where\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,\n    V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"docblock\"><p>实现了HashMap的几个基本方法</p>\n</div><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#104-109\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.new\" class=\"fn\">new</a>() -&gt; Self</h4></section><section id=\"method.insert\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#111-114\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.insert\" class=\"fn\">insert</a>(&amp;mut self, mark: K, code_object: V)</h4></section><section id=\"method.extend\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#116-122\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.extend\" class=\"fn\">extend</a>(&amp;mut self, map: Self)</h4></section><section id=\"method.get\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#124-131\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.get\" class=\"fn\">get</a>&lt;Q&gt;(&amp;self, mark: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Q</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;V</a>&gt;<div class=\"where\">where\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,\n    Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section><section id=\"method.get_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#133-140\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.get_mut\" class=\"fn\">get_mut</a>&lt;Q&gt;(&amp;mut self, mark: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Q</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut V</a>&gt;<div class=\"where\">where\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,\n    Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section><section id=\"method.contains_key\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#142-148\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.contains_key\" class=\"fn\">contains_key</a>&lt;Q&gt;(&amp;self, mark: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Q</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a><div class=\"where\">where\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,\n    Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section><section id=\"method.iter\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#150-152\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.iter\" class=\"fn\">iter</a>(&amp;self) -&gt; impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;K</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;V</a>)&gt;</h4></section><section id=\"method.keys\" class=\"method\"><a class=\"src rightside\" href=\"src/pyrev/core/common.rs.html#154-156\">source</a><h4 class=\"code-header\">pub fn <a href=\"pyrev/core/common/struct.OrderMap.html#tymethod.keys\" class=\"fn\">keys</a>(&amp;self) -&gt; impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;K</a>&gt;</h4></section></div></details>",0,"pyrev::core::parse_opcode::CodeObjectMap"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()