module.exports = {
    '@tags':['StackOverflow'],
    'Test StackOverflow Search' : function (browser) {
      //
      const SignupDisplayName='NightNightNight45';
      const SignupEmail='d5168232@urhen.com';
      const SignUpPassword='Nightwatch2020!';
      
      const LoginEmail = 'threeminusfive@gmail.com';
      const LoginPassword = 'Nightwatch2020!';

      const SignUpField1 = 'input[name="display-name"]';
      const SignUpField2 = 'input[name="email"]';
      const SignUpField3 = 'input[name="password"]';      

     //Login doesn't require display name, only email and password
      const LoginField1 = 'input[name="email"]';
      const LoginField2 = 'input[name="password"]';



      browser
        .url('https://stackoverflow.com')
        .waitForElementVisible('body')
        .assert.titleContains('Stack Overflow - Where Developers Learn, Share, & Build')
        .assert.visible('input[type=text]') // look for searchbar
        .setValue('input[type=text]', ['Automated Testing', browser.Keys.ENTER]) // search for Automated Testing
        .waitForElementVisible('#mainbar') // wait for main content of website to show
        .assert.containsText('#question-summary-12135309', 'REST API') // REST API will show in results after searching Automated Testing
        .click('.question-hyperlink') // click question pertaining to REST API
        .saveScreenshot('tests_output/StackOverflow.png') // prints screenshot of page for evidence
        .click('.-ctas a:nth-child(2)')
        .pause(2000)
        //Sign up for StackOverflw
        //.url('https://stackoverflow.com/users/signup?ssrc=head&returnurl=%2fusers%2fstory%2fcurrent')
       // .setValue(SignUpField1, SignupDisplayName)
        //.setValue(SignUpField2, SignupEmail)
       // .setValue(SignUpField3, SignUpPassword)
        //.click('button[name=submit-button]') 
        //.saveScreenshot('tests_output/StackOverflowLogin.png')
        //Login to StackOverflow
        //This will use the existing account we created earlier
        .url('https://stackoverflow.com/users/login?ssrc=head&returnurl=https%3a%2f%2fstackoverflow.com%2f')
        .setValue(LoginField1, LoginEmail)
        .setValue(LoginField2, LoginPassword)
        //Submits Login Button
        .click('button[name=submit-button]')
        //pause and fill out Captcha
        .pause(5000)
        .waitForElementVisible('body')
        .assert.containsText('#mainbar', 'Top Questions') // ensure logged in, Top questions will appear at heading
        .click('#mainbar .grid--cell a:nth-child(3)') //Click on the Hot Tab
       // .pause(5000)
        .click('.question-hyperlink') //Click on the 5th question
        .pause(3000)
        .moveToElement('.js-site-switcher-button', 15, 15) //move to top-right menu button
        .pause(1000)
        .click('.js-site-switcher-button') // Click on menu button
        .setWindowSize(400,600) // Set screen size to max
        .windowMaximize()
        
        
        .useXpath() // to log out
        .click("//*[contains(text(), 'log out')]")    
        .useCss()
        .pause(2300)
        // clicks log out button
        .click('.s-btn__primary')         
        .end();
        
        
        
    }
   
  };