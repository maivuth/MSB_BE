<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Registration Account</name>
   <tag></tag>
   <elementGuidId>50e5e051-01f7-470a-84f4-908d01aac256</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIxNTU5NDYsImlhdCI6MTcxODU1NTk0NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.ALlFT0w8YZ5F4hYG-r9ormDS-EllvdrNCW5vLrJBBK9e6rX9TamFjGUvDktXkP7MEs2TnsLg80IUkOQcytMN7A</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;registrationAccount\&quot;: {\n    \&quot;province\&quot;: \&quot;HN01\&quot;,\n    \&quot;district\&quot;: \&quot;HK01\&quot;,\n    \&quot;branch\&quot;: \&quot;MainBranch\&quot;,\n    \&quot;accountNumber\&quot;: \&quot;9876543210\&quot;,\n    \&quot;selectedAccountType\&quot;: \&quot;RANDOM_ACCOUNT\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d468b3a0-b68f-4a6b-bdef-cd6419298d23</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIxNTU5NDYsImlhdCI6MTcxODU1NTk0NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.ALlFT0w8YZ5F4hYG-r9ormDS-EllvdrNCW5vLrJBBK9e6rX9TamFjGUvDktXkP7MEs2TnsLg80IUkOQcytMN7A</value>
      <webElementGuid>2b13268b-67d7-4add-b87c-fabc9d1edfa1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://msb.kmsmoba.com/dev/api/user/account-registration-details</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
