module.exports = {
    '@tags':['Supreme'],
    'Supreme' : function(browser) {
       
        const formField1 = 'Anthony Prater';
        const formField2 = 'threeminusfive@gmail.com';
        const formField3 = '301-706-9630';
       const formField4 = '14601 Stratfield Cir';
        const formField5 = '20707-5502';
        const formField6 = 'Laurel';
       const formField7 = '4430 4730 8287 4448';
       const formField8 = '053';


       const formFieldSelector1 ='input[name="order[billing_name]"]';
        const formFieldSelector2 ='input[name="order[email]"]';
        const formFieldSelector3 ='input[name="order[tel]"]';
        const formFieldSelector4 ='input[name="order[billing_address]"]';
        const formFieldSelector5 ='input[name="order[billing_zip]"]';
       const formFieldSelector6='input[name="order[billing_city]"]';
        const formFieldSelector7='input[id="rnsnckrn"]'; //CC number
        const formFieldSelector8 ='input[id="orcer"]'; //CVV

        browser

            .url('https://supremenewyork.com/shop/all/t-shirts')
            
            
           // .waitForElementVisible('#wrap')
           // .assert.visible('#wrap')
          
            .useXpath()
            .click("//*[contains(text(), 'Bite')]")
            // FOR BOX LOGO USE THIS
            //.click(" [contains(text(), 'Box Logo')]")  
            .useCss()
            
            
            
            .click('#ard-rrmove-brttons .button')
            //.pause(900)
            //.saveScreenshot('tests_output/Suprema.png')
            
           // .click('select[id="s"] option[value="74724"]')

            
           // .click('input[type="submit"]')
            //.pause(2300)
            .pause(1600)
            
           .useXpath()  
            
            .click("//*[contains(text(), 'checkout')]")
            .useCss()
            
            // fill out billing info
            .setValue(formFieldSelector1, formField1)
             .setValue(formFieldSelector2, formField2)
             .setValue(formFieldSelector3, formField3)
             .setValue(formFieldSelector4, formField4)
             .setValue(formFieldSelector5, formField5)
             //.pause(2500)
             //sometimes the form will autofill the city based on zip code
             .assert.not.containsText('input[name="order[billing_city]"]', 'Laurel', function(result){
                    if (result.value == true){
                          
                        
                        console.log('City filled by script')
                    }
                    else{
                        browser.clearValue('input[name="order[billing_city]"')
                        browser.setValue(formFieldSelector6, formField6)
                        console.log('City done')
                    }




             })
             // Select state - Maryland
             .click('.order_billing_state option[value=MD]')
             //remember address for future orders
             .click('#store-address')
           // .setValue(formFieldSelector6, formField6)
             .setValue(formFieldSelector7, formField7)
             .setValue(formFieldSelector8, formField8)
             .useXpath()
            .click("//*[contains(text(), 'I have read and agree to the ')]")
            .useCss()
            

           // .click('input[type=checkbox')
             .pause(2000)
            // wait for captcha
            // solve captcha

        
             
            
             
             .pause(7000)
            .end();
        


    }








}