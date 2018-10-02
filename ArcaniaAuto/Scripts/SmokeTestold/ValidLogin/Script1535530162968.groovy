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

WebUI.openBrowser('https://arcania-integ.astek.mu/Login')

WebUI.navigateToUrl('https://arcania-integ.astek.mu/Login')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

WebUI.setText(findTestObject('Page_Arcania - Log In/input_Password'), 'b1')

WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

WebUI.waitForElementPresent(findTestObject('Page_Arcania - Dashboard/h1_Dashboard'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/i_icon-home'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/i_icon-crm'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/i_icon-leads'), 0)

WebUI.click(findTestObject('Page_Arcania - Dashboard/i_icon-leads'))

WebUI.verifyTextPresent('LEADS', false)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/i_icon-intranet'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/i_icon-bi'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/MenubarToDoIcon'), 0)

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenubarToDoIcon'))

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenubarTodo_SubMenu_Go to List'), FailureHandling.STOP_ON_FAILURE)

try {
    WebUI.verifyTextPresent('SCHEDULE', false)
}
finally { 
    WebUI.takeScreenshot(FailureHandling.CONTINUE_ON_FAILURE)
}

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenubarSchedulesIcon', [('variable') : '']))

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenubarTodo_SubMenu_Go to List'))

WebUI.verifyTextPresent('SCHEDULE', false)

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenuBarLanguageIcon'))

WebUI.click(findTestObject('Page_Arcania - Dashboard/MenubarLanguage_SubMenuFrench'))

WebUI.dismissAlert(FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Arcania - Log In/Account'))

WebUI.click(findTestObject('Page_Arcania - Log In/Logout'))

WebUI.closeBrowser()

