# Nightwatch Automation

In this project we are using Nightwatch to perform end-to-end testing.

## Installation

Download your favorite IDE. In this case I'm using [Microsoft VS Code](https://code.visualstudio.com/download).

Install [git](https://git-scm.com/) for version control

Install [Node.js and npm](https://nodejs.org/en/) to run tests

### Setup
Open a new empty folder (File -> New Folder) and give it a name (e.g. tests).

Open the terminal using View -> Terminal or Ctrl + `

## In the terminal....

Use git to initialize a repository

```bash
git init
```

Use npm to create a packages.json file

```bash
npm init -y
```

In packages.json replace the entire "no test specified" line with "nightwatch", remvoing quotes.





Use npm to install nightwatch

```bash
npm install nightwatch --save-dev
```

## Outside of the terminal...

Creata a configuration file called nightwatch.conf.js (Right click -> New file)

Copy the contents of [Nightwatchjs Configuration](https://nightwatchjs.org/gettingstarted/configuration/) into nightwatch.conf.js


Your nightwatch.conf.js file should look like:
```bash
{
  "src_folders" : ["tests"],

  "webdriver" : {
    "start_process": true,
    "server_path": "node_modules/.bin/chromedriver",
    "port": 9515
  },

  "test_settings" : {
    "default" : {
      "desiredCapabilities": {
        "browserName": "chrome"
      }
    }
  }
}
```

## Inside the terminal...

Install chromedriver in order to run tests in the browser
```bash
npm i chromedriver -y
```



If using Windows, in the nightwatch.conf.js, change server_path to: 

```bash
"server_path": "node_modules/chromedriver/lib/chromedriver/chromedriver"

or "server_path": "node_modules/chromedriver/lib/chromedriver/chromedriver.exe"

```

If using Mac, move onto next step




## Setting up your first test

In your empty folder (tests), create a javascript file and rename it (e.g. firstTest.js), or create a Javascript file and name it tests/FirstTest.js

``` Sample Layout

module.exports=
{
   '@tags':['newTest'],
"check Google"(browser){

    const searchTerms = "Node.js";
    const searchFiller = 'input[class="gLFyf gsfi"]';
browser
.url('https://google.com')
    .pause(2500)
    .setValue(searchFiller, searchTerms)
    .keys(browser.Keys.ENTER)
    .pause(3000)
    }
}
```

Every javascript test file must start with module.exports{} and have a browser object (browser or client, whichever you preffer)

Frequently refer to the [NightwathJS Commands](https://nightwatchjs.org/api/commands/) page to view commands you can do

For example,
browser.url(https://google.com) tells the browser to navigate to google. You can use any website you want to test 
browser.pause(2500) pauses the test for 2500 milliseconds
browser.setValue(searchFiller, searchTerms) sets the search bar (I named it searchFIller) with the term I want to search for (Node.js)
browser.keys(browser.Keys.ENTER) presses the ENTER key on the keyboard (i.e. submits the form)
browser.assert.containsText('.TbwUpd', 'https://nodejs.org') checks whether the seabheading contains the node.js website link


Assertions print in the terminal and easily communicate what on the test passed versus what failed




## Usage
 In the terminal...
```
npm test --tag "Place tag here without quotes (e.g. npm test --tag newTest) -> press Enter Key

To test all tests at the same time:
npm test -> Press Enter Key
```


## Contributing
1. Fork the Project
2. Create your Feature Branch (git checkout -b feature/AmazingFeature)
3. Commit your Changes (git commit -m 'added some tests')
4. Push to the Branch (git push origin projects)
5. Open a Pull Request

## License
[MIT](https://choosealicense.com/licenses/mit/)