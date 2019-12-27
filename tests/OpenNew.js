module.exports = {

    '@tags':['OpenNew'],
    'Open New' : function(browser){
        
        
        browser
            .execute(function (url1, window1){
                window.open(supremenewyork.com/checkout, window1, "height=1024,width=768");
            }, [url1, window1]);




    }
}