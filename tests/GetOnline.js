module.exports = {

    '@tags':['GetOnline'],
    'GetOnline' : function(browser){
        // this test will test whether the user is online or offline
        //If online, do nothing; the user is connected
        // If offline, the browser will connect by clicking Accept button

        browser
            .url('http://neverssl.com') //connect to non secured site for captive wifi portal to appear
            .pause(2500) //wait for redirect

           


           
            
           browser.element('css selector','.container', results =>{
            if (results.status != -1){
                    console.log('Connected')
                }
                else{
                   console.log('Connecting to internet...')

                   browser
                   // .moveToElement('input[type="button"]',15,15) // scrll element int view
                    .click('input[value="Accept"]')
                    .pause(1000)
                   console.log('Connected!, browse the Internet')
                   browser.click()


                }
            })

        }
    }
            
    
            
            
            
            
            
          
