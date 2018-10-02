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

not_run: WebUI.waitForElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/i_icon-leads'), 10)

WebUI.click(findTestObject('Page_Arcania - Dashboard/i_icon-leads'))

WebUI.waitForElementClickable(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeads'))

WebUI.waitForPageLoad(15)

WebUI.delay(5)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/Leads_CreateLead'), 10)

WebUI.verifyElementClickable(findTestObject('Page_Arcania - Leads/Agent Leads/Leads_CreateLead'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/Leads_CreateLead'))

not_run: CustomKeywords.'newpackage.newkeyword.clickElement'(findTestObject('Page_Arcania - Leads/Agent Leads/Leads_CreateLead'))

not_run: WebUI.waitForElementPresent(findTestObject('Page_Arcania - Dashboard/AgentQuote'), 10)

not_run: WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/AgentQuote'), 10)

not_run: WebUI.click(findTestObject('Page_Arcania - Dashboard/AgentQuote'))

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

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/label_Origin service'), 10)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/label_Origin service'))

