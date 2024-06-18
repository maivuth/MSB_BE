package core.database

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import groovy.sql.Sql
import com.kms.katalon.core.util.KeywordUtil

public class Postgresql {

	@Grab(group='org.postgresql', module='postgresql', version='42.7.3')

	@Keyword
	def connectPostgreSql(String url, String port, String dbName, def dbDriver = "org.postgresql.Driver", String username, String password) {
		String connectionString = "jdbc:postgresql://" + url + ":" + port + "/" + dbName
		return Sql.newInstance(connectionString, username, password, dbDriver)
	}

	@Keyword
	def executeSQL(String url, String port, String dbName, def dbDriver = "org.postgresql.Driver", String username, String password, String query) {
		Sql sql = connectPostgreSql(url, port, dbName, dbDriver, username, password)
		try {
			sql.eachRow(query){ row -> KeywordUtil.logInfo('Data: ' + row.toString()) }
		}
		catch (Exception e) {
			KeywordUtil.logInfo( "Error executing SQL query: ${e.message}")
		}
		finally {
			sql.close()
		}
	}
}
