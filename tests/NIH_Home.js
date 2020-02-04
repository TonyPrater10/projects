module.exports = {
    '@disabled': true,
    '@tags': ['NIH_Home'],
    'NIH Home Page Search' : !function(browser) {
        browser
            .url('https://niaid.nih.gov')
            .setValue('#search-niaid', 'Virus')
            .click('#search-niaid-submit') // click search
            .assert.containsText('#block-niaid-drupal-theme-niaid-adaptive-system-main > div > div > div.view-content > div:nth-child(1) > div > h2 > a:nth-child(2)')
    }
}