<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Registration Account</name>
   <tag></tag>
   <elementGuidId>9fce2513-e0fe-4530-bc72-f5666676885b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bearerToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bfd71e72-384c-411b-b3b0-eaec88887428</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>77d4fe8c-28d0-458e-85df-5d37a3bc6042</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/account-registration-details</restUrl>
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
      <id>71fa7afc-728c-4199-b9b7-0fd0a5b115fe</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIyMTA1IiwicmVnaXN0cmF0aW9uX2lkIjoyMTA1LCJleHAiOjE3MjIzMjg2MDYsImlhdCI6MTcxODcyODYwNiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QSSkC3sARGOFZakq-NYriwkFzBHvR3CWviEWMETi99KzDWKRarahYeVi8lpUphgafPPgMcxl38N9_xLBhoYiJg'</defaultValue>
      <description>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w</description>
      <id>97e48da7-e9f5-4571-aa4e-ccc41395d382</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

try {
	WS.verifyResponseStatusCode(response, 200)
	
	assertThat(response.getStatusCode()).isEqualTo(200)
String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
  &quot;title&quot;: &quot;registrationAccount&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;registrationAccount&quot;: {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;id&quot;: {&quot;type&quot;: &quot;number&quot;},
        &quot;province&quot;: {&quot;type&quot;: &quot;string&quot;},
        &quot;district&quot;: {&quot;type&quot;: &quot;string&quot;},
        &quot;branch&quot;: {&quot;type&quot;: &quot;string&quot;},
        &quot;accountNumber&quot;: {&quot;type&quot;: &quot;string&quot;},
        &quot;selectedAccountType&quot;: {&quot;type&quot;: &quot;string&quot;},
        &quot;enterpriseAccountId&quot;: {&quot;type&quot;: &quot;number&quot;}
        },
        &quot;required&quot;: [&quot;id&quot;, &quot;province&quot;, &quot;district&quot;, &quot;branch&quot;,
                    &quot;accountNumber&quot;, &quot;selectedAccountType&quot;,
                    &quot;enterpriseAccountId&quot;]
      }
    }
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
}
catch(Exception e) {
	throw &quot;API failed&quot;
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
