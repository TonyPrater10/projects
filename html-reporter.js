var fs = require('fs');
var path = require('path');
var handlebars = require('handlebars');


module.exports = {
  write : function(results, options, done) {

    var fileCount =0;

    

    // var reportFilename = options.filename_prefix + (Math.floor(Date.now() / 1000)) + '.html';
    var reportFilename = options.filename_prefix + process.argv[3]; //process.argv lets us know current script running. run console.log(process.argv) to see current scripts
    var reportFilePath = path.join(__dirname, options.output_folder, reportFilename);
    while (fs.existsSync(reportFilePath))
    {
      fileCount++;
      reportFilePath = reportFilePath + '' + '(' + fileCount + ')' + '.html';
    }
    
  

    

    // read the html template
    fs.readFile('html-reporter.hbs', function(err, data) {
      if (err) throw err;

      var template = data.toString();

      // merge the template with the test results data
      var html = handlebars.compile(template)({
        results   : results,
        options   : options,
        timestamp : new Date().toString(),
        browser   : options.filename_prefix.split('_').join(' ')
      });

      // write the html to a file
      fs.writeFile(reportFilePath, html, function(err) {
        if (err) throw err;
        console.log('Report generated: ' + reportFilePath);
        console.log('FileCount: ' + fileCount);
        //console.log(process.argv[3])
        done();
      });
    });
  }
};