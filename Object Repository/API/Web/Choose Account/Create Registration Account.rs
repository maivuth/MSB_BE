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
  &quot;text&quot;: &quot;{\n  \&quot;registrationAccount\&quot;: {\n    \&quot;province\&quot;: \&quot;${province}\&quot;,\n    \&quot;district\&quot;: \&quot;${district}\&quot;,\n    \&quot;branch\&quot;: \&quot;@{branch}\&quot;,\n    \&quot;accountNumber\&quot;: \&quot;${accountNumber}\&quot;,\n    \&quot;selectedAccountType\&quot;: \&quot;${ccountType}\&quot;\n  }\n}&quot;,
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
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>81498203-5b73-4fba-85d4-1d08f2a0c45b</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>66134406-5048-44d9-a4cf-40bff8b0bcdd</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fa716b9-8585-4b40-9ab7-561123cd0501</id>
      <masked>false</masked>
      <name>province</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fbd05a2-1d08-48e1-bfc4-6d38e5f76515</id>
      <masked>false</masked>
      <name>district</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d8a24382-d58e-492c-8f46-3b20037625c9</id>
      <masked>false</masked>
      <name>branch</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>adbe2c5e-8f3d-4c64-b78d-8600dcdebd6b</id>
      <masked>false</masked>
      <name>accountNumber</name>
   </variables>
   <variables>
      <defaultValue>'SPECIAL_NUMBER'</defaultValue>
      <description>RANDOM_ACCOUNT</description>
      <id>4dc96eab-d282-4f7c-9566-60edaa6b3c78</id>
      <masked>false</masked>
      <name>accountType</name>
   </variables>
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
