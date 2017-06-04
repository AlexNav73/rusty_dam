$(function() {
    $('#tree').jstree({
        core: {
            data: {
                url: function (node) {
                    return node.id === '#' ?
                        "/classifications/parents" :
                        "/classifications/children";
                },
                data: function (node) {
                    return {
                        id: node.id === '#' ?
                            "bd3363fb-0fe1-4628-a24f-fdda8ef06b20" :
                            node.id
                    };
                }       
            },
            themes: {
                name: "default-dark",
                dots: false,
                icons: true
            }
        }
    });
});
