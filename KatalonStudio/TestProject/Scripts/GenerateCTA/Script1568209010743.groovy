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

WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d%2Da8f4%2D4e9f%2D87b9%2D81e30ad91b15%3Ahttps%3A%2F%2Fedrms%2Ddev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%252Fdisplayform%252F2000%252F517082%252F517083%252F79810217%252F79810218%252F79940375%252F%253Fviewid%253D79968935%2526readonly%253Dfalse%2526sedit%253Dfalse%2526objId%253D79973261%2526objAction%253DEditForm%2526viewType%253D1%2526nexturl%253D%25252Flivelink%25252Fllisapi%25252Edll%25253Ffunc%25253Dll%252526objId%25253D79810218%252526objAction%25253Dbrowse%252526viewType%25253D1&PostTicket=true&PostParams=true&ux_version=1&PreserveFragment=true')

WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_New AgreementAmend Agreement__1_1_2_1'))

WebUI.selectOptionByValue(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/ddAgreementType'), 'Clinical Trial Agreement', 
    true)

WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_YesNo__1_1_20_1'))

WebUI.selectOptionByValue(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/ddCompany1'), 'ABT Holding Company', 
    true)

WebUI.selectOptionByValue(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/ddProduct1_1'), 'Aflunov \\(MF59-adjuvanted\\)', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/ddRO'), '68255853', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/ddMO'), '16400079', 
    true)

WebUI.setText(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 'Initiated by Katalon')

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/span_Instructions_InstSpan'))

WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_Comments_btnButton1'))

WebUI.click(findTestObject('Object Repository/Page_Workflow Step DAIT CTA Agreement Workflow Map - Shalini Tahiliani - 09112019 - WorkID/input_Your workflow has been successfully initiated_processButton'))

//WebUI.closeBrowser()

