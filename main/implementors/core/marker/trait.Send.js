(function() {var implementors = {
"interner":[["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/global/struct.GlobalPool.html\" title=\"struct interner::global::GlobalPool\">GlobalPool</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    &lt;T as Poolable&gt;::Boxed: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::global::GlobalPool"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/global/struct.StaticPooledString.html\" title=\"struct interner::global::StaticPooledString\">StaticPooledString</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::global::StaticPooledString"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/global/struct.StaticPooledBuffer.html\" title=\"struct interner::global::StaticPooledBuffer\">StaticPooledBuffer</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::global::StaticPooledBuffer"]],["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/global/struct.StaticPooledPath.html\" title=\"struct interner::global::StaticPooledPath\">StaticPooledPath</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::global::StaticPooledPath"]],["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/shared/struct.SharedPool.html\" title=\"struct interner::shared::SharedPool\">SharedPool</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    &lt;T as Poolable&gt;::Boxed: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::shared::SharedPool"]],["impl&lt;P, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"interner/struct.Pooled.html\" title=\"struct interner::Pooled\">Pooled</a>&lt;P, S&gt;<span class=\"where fmt-newline\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    &lt;P as PoolKindSealed&lt;S&gt;&gt;::Pooled: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>",1,["interner::Pooled"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()