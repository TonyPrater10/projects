require('dotenv').config();

module.exports = 
{
    "src_folders" : ["tests"],
    "output_folder": "reports",
     "page_objects_path" : ["page_objects"],

  
    "webdriver" : {
      "start_process": true,
      "server_path": "node_modules/chromedriver/lib/chromedriver/chromedriver",
      "port": 9515
    },
  
    "test_settings" : {
      "default" : {
        "desiredCapabilities": {
          "browserName": "chrome"
        }
      }
    }
  };