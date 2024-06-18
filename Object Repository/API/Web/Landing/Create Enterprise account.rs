<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Enterprise account</name>
   <tag></tag>
   <elementGuidId>8d49a525-abef-4f66-8f11-244762c13b22</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;enterpriseAccount\&quot;: {\n    \&quot;consent\&quot;: ${consent},\n    \&quot;taxCode\&quot;: \&quot;${taxCode}\&quot;,\n    \&quot;companyName\&quot;: \&quot;${companyName}\&quot;,\n    \&quot;contactPersonName\&quot;: \&quot;${contactName}\&quot;,\n    \&quot;contactPersonPhone\&quot;: ${contactPhone},\n    \&quot;contactPersonEmail\&quot;: \&quot;${contactEmail}\&quot;,\n    \&quot;supportStaffEmail\&quot;: \&quot;${supportEmail}\&quot;,\n    \&quot;openingAccountPurpose\&quot;: \&quot;${openingPurpose}\&quot;\n  }\n}&quot;,
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
      <webElementGuid>6edaa900-5be2-4c2b-ae6d-6ec31699d25f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrl}/user/enterprise-accounts</restUrl>
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
      <id>abcf8599-8df0-4673-ac68-15a8d5e673cc</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>ba6c7611-973c-42d2-b580-d964e41648d1</id>
      <masked>false</masked>
      <name>consent</name>
   </variables>
   <variables>
      <defaultValue>'5555923668'</defaultValue>
      <description>taxCode</description>
      <id>050048c5-e2e9-465e-95be-f9ebd5fe3e21</id>
      <masked>false</masked>
      <name>taxCode</name>
   </variables>
   <variables>
      <defaultValue>'API Test Comp'</defaultValue>
      <description>companyName</description>
      <id>dda787d7-19bc-4e03-a815-c09df1e7f78e</id>
      <masked>false</masked>
      <name>companyName</name>
   </variables>
   <variables>
      <defaultValue>'Contact Testing'</defaultValue>
      <description>contactPersonName</description>
      <id>df911882-f734-494a-ae5a-23ead8b124df</id>
      <masked>false</masked>
      <name>contactName</name>
   </variables>
   <variables>
      <defaultValue>'8439055668'</defaultValue>
      <description>contactPersonPhone</description>
      <id>a97e7f8e-0648-416b-b1ab-16ce487f7bef</id>
      <masked>false</masked>
      <name>contactPhone</name>
   </variables>
   <variables>
      <defaultValue>'contact@estmail.com'</defaultValue>
      <description>contactPersonEmail</description>
      <id>1e1565af-3e2e-4306-b069-7b802f73ad61</id>
      <masked>false</masked>
      <name>contactEmail</name>
   </variables>
   <variables>
      <defaultValue>'support@testmail.com'</defaultValue>
      <description>supportStaffEmail</description>
      <id>b629060e-6b93-452d-8fe5-59dd2012e7c1</id>
      <masked>false</masked>
      <name>supportEmail</name>
   </variables>
   <variables>
      <defaultValue>'PC01'</defaultValue>
      <description>openingAccountPurpose</description>
      <id>ad8baa5e-3b06-447e-865b-cfe7a0abe967</id>
      <masked>false</masked>
      <name>openingPurpose</name>
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
	
	assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))
	
	String jsPass =
	&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Root&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;accessToken&quot;: {
      &quot;type&quot;: &quot;string&quot;
		},
      &quot;refreshToken&quot;: {
        &quot;type&quot;: &quot;string&quot;
		}
},
  &quot;required&quot;: [
    &quot;accessToken&quot;,
    &quot;refreshToken&quot;
  ]
};
&quot;&quot;&quot;
	boolean successful = WS.validateJsonAgainstSchema(response,jsPass)
}catch(Exception e) {
	throw &quot;API invalid&quot;
}
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
