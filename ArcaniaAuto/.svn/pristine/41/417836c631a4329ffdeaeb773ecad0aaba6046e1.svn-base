import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

'Open Browser'
WebUI.openBrowser('https://arcania-integ.astek.mu/Login')

'Navigate to link'
WebUI.navigateToUrl('https://arcania-integ.astek.mu/Login')

'Maximize window - Full screen'
WebUI.maximizeWindow()

'Set username'
WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

'Set password'
WebUI.setText(findTestObject('Page_Arcania - Log In/input_Password'), 'b1')

'Click sign in button'
WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

'Wait for page load\r\n'
WebUI.waitForPageLoad(10)

'Check if Dashboard appears'
WebUI.waitForElementPresent(findTestObject('Page_Arcania - Dashboard/h1_Dashboard'), 10)

CustomKeywords.'newpackage.newkeyword.clickElement'(findTestObject('Page_Arcania - Dashboard/CreateLeadBtn'))

'Check if Create button appears'
not_run: WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/CreateLeadBtn'), 10)

'Check if Create button is clickable'
not_run: WebUI.verifyElementClickable(findTestObject('Page_Arcania - Dashboard/CreateLeadBtn'))

'Click Create button'
not_run: WebUI.click(findTestObject('Page_Arcania - Dashboard/CreateLeadBtn'))

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Dashboard/AgentQuote'), 10)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/AgentQuote'), 10)

WebUI.click(findTestObject('Page_Arcania - Dashboard/AgentQuote'))

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads_AgentName'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads_AgentName'))

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/li_Astek Mauritius Ltd'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/li_Astek Mauritius Ltd'), FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads_ContactPerson'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads_ContactPerson'))

'Verify if DropDown is clickable'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/Hidden_TextField_ContactPerson'), 
    10)

WebUI.setText(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/Hidden_TextField_ContactPerson'), 
    'Roy')

WebUI.sendKeys(findTestObject(null), Keys.chord(Keys.ENTER))

not_run: WebUI.waitForElementVisible(findTestObject('null'), 10)

'Scroll to the desire element'
not_run: WebUI.scrollToElement(findTestObject('null'), 10)

not_run: WebUI.focus(findTestObject('null'))

'Click the desired element\r\n'
not_run: WebUI.waitForElementClickable(findTestObject('null'), 10)

not_run: WebUI.click(findTestObject('null'))

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/label_Assistance'), 
    10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/label_Assistance'))

WebUI.verifyElementClickable(findTestObject('Page_Arcania - Leads/Agent Leads/Scope_of_Service_Next'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Scope_of_Service_Next'))

WebUI.waitForPageLoad(5)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/label_Packing'), 
    10)

WebUI.focus(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/label_Packing'))

not_run: WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_div_ Mandatory fields'), 
    10)

not_run: WebUI.focus(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_div_ Mandatory fields'))

not_run: WebUI.verifyElementPresent(findTestObject('null'), 10)

not_run: WebUI.focus(findTestObject('null'))

not_run: WebUI.click(findTestObject('null'))

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_input_Address.Steet'), 
    10)

WebUI.focus(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_input_Address.Steet'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_input_Address.Steet'))

WebUI.setText(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_input_Address.Steet'), 
    'CyberTower 1, Ebene')

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/DropDownCountry'))

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequestSpan_Address_Country_container'))

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_span_Address_Country_container'))

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/dropdddd'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/dropdddd'))

WebUI.setText(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/Hidden_TextField'), 'France')

WebUI.sendKeys(findTestObject(null), Keys.chord(Keys.ENTER))

not_run: WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/DropDownCountry'), 
    10)

not_run: WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/DropDownCountry'), 
    10)

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/DropDownCountry'))

not_run: WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'), 
    10)

not_run: WebUI.waitForElementVisible(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'), 
    10)

not_run: WebUI.scrollToElement(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'), 
    10)

not_run: WebUI.focus(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'))

not_run: WebUI.waitForElementClickable(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'), 
    10)

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_France'))

WebUI.setText(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequestInput_Address.City'), 
    'Ebene')

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/drop_Price'), 
    10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/drop_Price'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/droppricearrowdown'))

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/droppricebig'))

not_run: WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/droppricehidden'))

not_run: WebUI.setText(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/CreateNewRateRequest_input_PricingMethod'), 
    'Flat')

not_run: WebUI.sendKeys(findTestObject(null), Keys.chord(Keys.ENTER))

WebUI.waitForElementVisible(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_Per 100 lbs'), 
    10)

WebUI.scrollToElement(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_Per 100 lbs'), 10)

WebUI.focus(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_Per 100 lbs'))

WebUI.waitForElementClickable(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_Per 100 lbs'), 
    10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/li_Per 100 lbs'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Page_Arcania - NewRateRequest/button_Submit request'))

