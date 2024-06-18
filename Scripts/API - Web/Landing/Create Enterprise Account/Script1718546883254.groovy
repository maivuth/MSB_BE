import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

//Random taxCode and contactPhone
taxCode = CustomKeywords.'com.msb.utils.StringHelper.randomNumber'('1000000000', '9999999999')
contactPhone = CustomKeywords.'com.msb.utils.StringHelper.randomNumber'('1000000000', '9999999999')
WebUI.comment(taxCode.toString())
WebUI.comment(contactPhone.toString())

def response = WS.sendRequest(findTestObject('API/Web/Landing/Create Enterprise account', 
	[('baseUrl') : baseUrl,
	 ('consent') : true, 
	 ('taxCode') : taxCode, 
	 ('companyName') : companyName, 
	 ('contactName') : contactName, 
	 ('contactPhone') : contactPhone, 
	 ('contactEmail') : contactEmail, 
	 ('supportEmail') : supportEmail, 
	 ('openingPurpose') : openingPurpose]))

def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
WebUI.comment(jsonResponse.toString())
try{
	if(WS.verifyResponseStatusCode(response, 200)) {
		WebUI.comment('API Passed')
		'Extract data from the JSON response'
		def resId = jsonResponse.enterpriseAccount.id.toString()
		WebUI.comment(resId)
	}
}
catch(Exception e) {
	WebUI.comment("API failed with error ${response.getStatusCode().toString()}")
	WebUI.comment(jsonResponse.code.toString())
	WebUI.comment(jsonResponse.message.toString())
}
