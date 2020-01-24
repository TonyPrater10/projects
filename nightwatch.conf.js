require('dotenv').config();

module.exports = 
{
    "src_folders" : ["tests"],
    "output_folder": "reports",
     //"page_objects_path" : ["pageobjects"],

  
    "webdriver" : {
      "start_process": true,
      "server_path": "node_modules/chromedriver/lib/chromedriver/chromedriver",
      "port": 9515
    },
  
    "test_settings" : {
      "default" : {
          'screenshots': {
            'enabled':true,
            'on_failure':true,
            'on_error':true,
            'path':'tests_output/screenshots'
          },
        "desiredCapabilities": {
          "browserName": "chrome",

        }
      }
    }
  };