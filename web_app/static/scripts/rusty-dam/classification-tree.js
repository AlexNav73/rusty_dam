$(function() {
    //function getTree() {
        //var data = [
            //{
                //text: "Parent 1",
                //nodes: [
                    //{
                        //text: "Child 1",
                        //nodes: [
                            //{
                                //text: "Grandchild 1"
                            //},
                            //{
                                //text: "Grandchild 2"
                            //}
                        //]
                    //},
                    //{
                        //text: "Child 2"
                    //}
                //]
            //},
            //{
                //text: "Parent 2"
            //}
        //];
        //return data;
    //}

    //var ajaxHelper = new document.utility.ajax();
    //$('#tree').treeview({
        //data: getTree(),
        //onNodeSelected: function(event, node) {
            //console.log(node);
            //ajaxHelper.getClassificationById(node.text);
        //}
    //});

    $('#tree').jstree();
});
