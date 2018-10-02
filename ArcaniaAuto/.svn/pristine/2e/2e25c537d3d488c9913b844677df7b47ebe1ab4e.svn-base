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

WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

WebUI.setEncryptedText(findTestObject('Page_Arcania - Log In/input_Password'), 'nYgPIvXAGJo=')

'Click on sign in Button\r\n'
WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

WebUI.verifyTextPresent('DASHBOARD', false)

WebUI.click(findTestObject('Page_Arcania - Log In/Account'))

WebUI.verifyElementText(findTestObject('Page_Arcania - Log In/AccountName'), 'Moris Nasseau')

WebUI.verifyElementText(findTestObject('Page_Arcania - Log In/DescriptionAccount'), 'b1@arc.com')

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Log In/Phonebook'), 0)

'Verifying Settings is present'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Log In/Settings'), 0)

'Log out'
WebUI.click(findTestObject('Page_Arcania - Log In/Logout'))

WebUI.closeBrowser()

