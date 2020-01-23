import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d-a8f4-4e9f-87b9-81e30ad91b15%3Ahttps%3A%2F%2Fedrms-dev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'tahilianis2')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    'Bh/QTNsNgjkD1i8nPLKQkA==')

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/span_Personal'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/span_My Workspace'))

WebUI.click(findTestObject('Page_tahilianis2 Home/span_Add Folder'))

WebUI.setText(findTestObject('Page_Add Folder/input_Name_name'), 'Demo4')

WebUI.click(findTestObject('Page_Add Folder/input_Create In_addButton'))

