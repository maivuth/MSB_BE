<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Provinces</name>
   <tag></tag>
   <elementGuidId>dc69c4fb-5710-4128-892f-ba7bef83be80</elementGuidId>
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
      <webElementGuid>aa5554f2-cf42-41ec-b398-1f4a465c807e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>8b49ba22-7ef6-4a30-85ad-172f60eef3bc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/locations/provinces</restUrl>
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
      <id>f4455f4c-ff7e-4e3d-8aec-9dc37a8ae0a7</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIxNTU5NDYsImlhdCI6MTcxODU1NTk0NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.ALlFT0w8YZ5F4hYG-r9ormDS-EllvdrNCW5vLrJBBK9e6rX9TamFjGUvDktXkP7MEs2TnsLg80IUkOQcytMN7A'</defaultValue>
      <description></description>
      <id>70f8930b-1460-4514-ba88-2f082bddb933</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

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
  &quot;title&quot;: &quot;provinceCodeList&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;provinceCodeList&quot;: {
      &quot;type&quot;: &quot;array&quot;,
      &quot;item&quot;: {
        &quot;additionalProperties&quot;: false,
        &quot;properties&quot; : {
            &quot;code&quot;: {&quot;type&quot;: &quot;string&quot;},
            &quot;description&quot;: {&quot;type&quot;: &quot;string&quot;}
        },
        &quot;required&quot;: [&quot;code&quot;, &quot;description&quot;]
      }
    },
    &quot;districtCodeList&quot;: {&quot;type&quot;: &quot;null&quot;},
    &quot;branchCodeList&quot;: {&quot;type&quot;: &quot;null&quot;}
  },
  &quot;required&quot;: [&quot;provinceCodeList&quot;]
}
&quot;&quot;&quot;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
}
catch (Exception e) {
	WebUI.comment(&quot;API invalid ${e.message()}&quot;)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
