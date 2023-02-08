var searchIndex = JSON.parse('{\
"interner":{"doc":"interner","t":[8,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,0,11,11,11,11,11,6,6,6,3,6,6,3,3,3,6,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,6,6,6,6,3,6,6,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["PoolKind","Pooled","borrow","borrow_mut","clone","clone_into","deref","eq","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","from","global","hash","into","ptr_eq","shared","to_owned","to_string","try_from","try_into","type_id","BufferPool","GlobalBuffer","GlobalPath","GlobalPool","GlobalString","PathPool","StaticPooledBuffer","StaticPooledPath","StaticPooledString","StringPool","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","from","from","from","from","get","get","get","get","get","get","get_static","get_static","get_static_with","get_static_with","get_static_with","into","into","into","into","new","pooled","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","with_capacity_and_hasher","with_capacity_and_hasher_init","with_hasher","with_hasher_init","BufferPool","PathPool","SharedBuffer","SharedPath","SharedPool","SharedString","StringPool","borrow","borrow_mut","clone","clone_into","default","eq","eq","fmt","from","get","get","get","into","pooled","to_owned","try_from","try_into","type_id","with_capacity_and_hasher","with_hasher"],"q":["interner","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","interner::global","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","interner::shared","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["A kind of interning pool. Currently there are only two …","A type that ensures only one copy of each value exists in …","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Global interning pools.","","Calls <code>U::from(self)</code>.","Returns true if <code>this</code> and <code>other</code> point to the exact same …","Shared interning pools that have no global state.","","","","","","A global byte buffer interning pool that manages …","A pooled buffer (<code>Vec&lt;u8&gt;</code>) that is stored in a <code>GlobalPool</code>.","A pooled path that is stored in a <code>GlobalPool</code>.","A global interned pool.","A pooled string that is stored in a <code>GlobalPool</code>.","A global path interning pool that manages <code>GlobalPath</code>s.","A lazily-initialized <code>GlobalBuffer</code> that stays allocated for …","A lazily-initialized <code>GlobalPath</code> that stays allocated for …","A lazily-initialized <code>GlobalString</code> that stays allocated for …","A global string interning pool that manages <code>GlobalString</code>s.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns a copy of an existing <code>GlobalBuffer</code> if one is found.","Returns a copy of an existing <code>GlobalPath</code> if one is found. …","Returns a copy of an existing <code>GlobalString</code> if one is found.","Returns a reference-counted clone of the contained …","Returns a reference-counted clone of the contained …","Returns a reference-counted clone of the contained …","Returns a static pooled buffer, which keeps the pooled …","Returns a static pooled string, which keeps the pooled …","Returns a static pooled buffer, which keeps the pooled …","Returns a static pooled path, which keeps the pooled path …","Returns a static pooled string, which keeps the pooled …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns a new instance using <code>RandomState</code> for the internal …","Returns a collection of the currently pooled items.","","","","","","","","","","","","","Returns a new instance using the provided hasher with …","Returns a new instance using the function to load the …","Returns a new instance using the provided hasher.","Returns a new instance using the function to load the …","A path interning pool that manages <code>SharedBuffer</code>s.","A path interning pool that manages <code>SharedPath</code>s.","A pooled buffer that belongs to a <code>BufferPool</code>.","A pooled path that belongs to a <code>PathPool</code>.","A shared pool of values that ensures only one copy of any …","A pooled string that belongs to a <code>StringPool</code>.","A string interning pool that manages <code>SharedString</code>s.","","","","","","","","","Returns the argument unchanged.","Returns a copy of an existing <code>SharedPath</code> if one is found. …","Returns a copy of an existing <code>SharedString</code> if one is found.","Returns a copy of an existing <code>SharedBuffer</code> if one is …","Calls <code>U::from(self)</code>.","Returns a collection of the currently pooled items.","","","","","Creates a new pool using the provided <code>BuildHasher</code> for …","Creates a new pool using the provided <code>BuildHasher</code> for …"],"i":[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,2,6,3,7,2,6,3,7,2,2,6,3,7,2,6,3,7,2,6,3,7,2,2,2,6,3,7,2,2,2,2,2,2,6,3,7,2,2,2,6,3,7,2,6,3,7,2,6,3,7,2,2,2,2,0,0,0,0,0,0,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14],"f":[0,0,[[]],[[]],[1,1],[[]],[1],[[[1,[2]],3],4],[[1,5],4],[[[1,[2]],6],4],[[[1,[2]],7],4],[[1,8],4],[[1,8],4],[1,4],[1,4],[[1,1],4],[[1,5],4],[[1,9],10],[[1,9],10],[[]],0,[1],[[]],[[1,1],4],0,[[]],[[],11],[[],12],[[],12],[[],13],0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[2,14],4],[[2,2],4],[[6,1],4],[[3,1],4],[[7,1],4],[[2,9],10],[[6,9],10],[[3,9],10],[[7,9],10],[[]],[[]],[[]],[[]],[[[2,[[16,[15]]]]],17],[[[2,[18]]],19],[[[2,[11]]],20],[6,20],[3,17],[7,19],[[[2,[[16,[15]]]]],3],[[[2,[11]],5],6],[[[2,[[16,[15]]]]],3],[[[2,[18]]],7],[[[2,[11]]],6],[[]],[[]],[[]],[[]],[[],2],[2],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],[[],13],[[],13],[[],13],[[],13],[21,2],[21,2],[[],2],[[],2],0,0,0,0,0,0,0,[[]],[[]],[14,14],[[]],[[],[[14,[22]]]],[[14,14],4],[[14,2],4],[[14,9],10],[[]],[[[14,[18]]],23],[[[14,[11]]],24],[[[14,[[16,[15]]]]],25],[[]],[14],[[]],[[],12],[[],12],[[],13],[21,[[14,[11]]]],[[],[[14,[11]]]]],"p":[[3,"Pooled"],[3,"GlobalPool"],[3,"StaticPooledBuffer"],[15,"bool"],[15,"str"],[3,"StaticPooledString"],[3,"StaticPooledPath"],[3,"Path"],[3,"Formatter"],[6,"Result"],[3,"String"],[4,"Result"],[3,"TypeId"],[3,"SharedPool"],[15,"u8"],[3,"Vec"],[6,"GlobalBuffer"],[3,"PathBuf"],[6,"GlobalPath"],[6,"GlobalString"],[15,"usize"],[3,"RandomState"],[6,"SharedPath"],[6,"SharedString"],[6,"SharedBuffer"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
