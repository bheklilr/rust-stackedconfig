var searchIndex = {};
searchIndex["stackedconfig"] = {"doc":"","items":[[3,"ConfigStack","stackedconfig","Combines multiple `serde_json::Value` objects together so they can be queried as a single, nested object.",null,null],[4,"Lookup","","Return value from looking up a path from a ConfigStack",null,null],[13,"Missing","","Indicates that the path did not resolve to a value",0,null],[13,"Found","","Contains the `serde_json::Value` found from a lookup",0,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"lookup"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"lookup"}],"output":{"name":"bool"}}],[11,"new","","",1,{"inputs":[],"output":{"name":"configstack"}}],[11,"with_path_sep","","",1,{"inputs":[{"name":"self"},{"name":"char"}],"output":{"name":"configstack"}}],[11,"push","","",1,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"configstack"}}],[11,"pop","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"get","","",1,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"lookup"}}],[11,"get_parts","","",1,{"inputs":[{"name":"self"},{"name":"vec"}],"output":{"name":"lookup"}}]],"paths":[[4,"Lookup"],[3,"ConfigStack"]]};
initSearch(searchIndex);
