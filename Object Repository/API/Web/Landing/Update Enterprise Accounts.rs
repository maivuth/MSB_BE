<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Enterprise Accounts</name>
   <tag></tag>
   <elementGuidId>a6d05880-658d-4411-b6bf-70def06c426c</elementGuidId>
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
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;enterpriseAccount\&quot;:\n        {\n            \&quot;consent\&quot;: ${consent},\n            \&quot;taxCode\&quot;: \&quot;${taxCode}\&quot;,\n            \&quot;companyName\&quot;: \&quot;${companyName}\&quot;,\n            \&quot;contactPersonName\&quot;: \&quot;${contactPersonName}\&quot;,\n            \&quot;contactPersonPhone\&quot;: \&quot;${contactPersonPhone}\&quot;,\n            \&quot;contactPersonEmail\&quot;: \&quot;${contactPersonEmail}\&quot;,\n            \&quot;supportStaffEmail\&quot;: \&quot;${supportStaffEmail}\&quot;,\n            \&quot;openingAccountPurpose\&quot;: \&quot;${openingAccountPurpose}\&quot;\n        }\n\n}\n\nString jsPass \u003d\n\t\&quot;\&quot;\&quot;\n{\n  \&quot;\\$schema\&quot;: \&quot;http://json-schema.org/draft-07/schema#\&quot;,\n  \&quot;title\&quot;: \&quot;Generated schema for Root\&quot;,\n  \&quot;type\&quot;: \&quot;object\&quot;,\n  \&quot;properties\&quot;: {\n    \&quot;enterpriseAccount\&quot;: {\n      \&quot;type\&quot;: \&quot;object\&quot;,\n      \&quot;properties\&quot;: {\n        \&quot;id\&quot;: {\n            \&quot;type\&quot;: \&quot;number\&quot;\n\t\t},\n        \&quot;taxCode\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;companyName\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;contactPersonName\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;contactPersonPhone\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;contactPersonEmail\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;supportStaffEmail\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;openingAccountPurpose\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;consent\&quot;: {\n            \&quot;type\&quot;: \&quot;boolean\&quot;\n\t\t},\n        \&quot;createdDate\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;lastModifiedDate\&quot;: {\n            \&quot;type\&quot;: \&quot;string\&quot;\n\t\t},\n        \&quot;registrationAccount\&quot;: {\n            \&quot;type\&quot;: \&quot;object\&quot;,\n            \&quot;properties\&quot;: {\n                \&quot;id\&quot;: {\n                    \&quot;type\&quot;: \&quot;number\&quot;\n                },\n                \&quot;province\&quot;: {\n                    \&quot;type\&quot;: \&quot;string\&quot;\n                },\n                \&quot;district\&quot;: {\n                    \&quot;type\&quot;: \&quot;string\&quot;\n                },\n                \&quot;branch\&quot;: {\n                    \&quot;type\&quot;: \&quot;string\&quot;\n                },\n                \&quot;accountNumber\&quot;: {\n                    \&quot;type\&quot;: \&quot;string\&quot;\n                },\n                \&quot;selectedAccountType\&quot;: {\n                    \&quot;type\&quot;: \&quot;string\&quot;\n                }\n            }\n\t\t},\n        \&quot;required\&quot;: [\n            \&quot;id\&quot;,\n            \&quot;taxCode\&quot;,\n            \&quot;companyName\&quot;,\n            \&quot;contactPersonName\&quot;,\n            \&quot;contactPersonPhone\&quot;,\n            \&quot;contactPersonEmail\&quot;,\n            \&quot;supportStaffEmail\&quot;,\n            \&quot;openingAccountPurpose\&quot;,\n            \&quot;openingAccountPurpose\&quot;,\n            \&quot;consent\&quot;,\n            \&quot;registrationAccount\&quot;\n        ]\n      }\n\t\t},\n},\n  \&quot;required\&quot;: [\n    \&quot;enterpriseAccount\&quot;\n  ]\n};\n\&quot;\&quot;\&quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fea898a5-5a59-44df-b2b2-5c763e34380a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>51cc14b8-1a7e-4a78-8eaa-9e21c028eb9a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
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
      <defaultValue>'https://msb.kmsmoba.com/dev/api'</defaultValue>
      <description></description>
      <id>b010876e-dd3b-417b-8938-0dcd0a035c5b</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>40f39593-64c8-4484-8eaa-ccab9e8be71a</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>d5fbd4f7-e5a8-4f4f-8d61-af6c64ff768a</id>
      <masked>false</masked>
      <name>consent</name>
   </variables>
   <variables>
      <defaultValue>'5655555550'</defaultValue>
      <description></description>
      <id>40b67605-589d-4734-a774-78fff8685ba0</id>
      <masked>false</masked>
      <name>taxCode</name>
   </variables>
   <variables>
      <defaultValue>'TEST01'</defaultValue>
      <description></description>
      <id>1907bbf0-9523-4d64-aa44-b37536f7c01c</id>
      <masked>false</masked>
      <name>companyName</name>
   </variables>
   <variables>
      <defaultValue>'Test User 01'</defaultValue>
      <description></description>
      <id>6e9c0364-7b08-4416-8274-1afa191430ce</id>
      <masked>false</masked>
      <name>contactPersonName</name>
   </variables>
   <variables>
      <defaultValue>'0909999000'</defaultValue>
      <description></description>
      <id>8300f391-8d7c-456b-bfdc-234f0ddf8427</id>
      <masked>false</masked>
      <name>contactPersonPhone</name>
   </variables>
   <variables>
      <defaultValue>'test@kms-technology.com'</defaultValue>
      <description></description>
      <id>77001b39-841c-4d6f-a39a-72de00c7cbcd</id>
      <masked>false</masked>
      <name>contactPersonEmail</name>
   </variables>
   <variables>
      <defaultValue>'test@kms-technology.com'</defaultValue>
      <description></description>
      <id>6feedb88-9279-457e-9c09-965b5140341f</id>
      <masked>false</masked>
      <name>supportStaffEmail</name>
   </variables>
   <variables>
      <defaultValue>'MP1'</defaultValue>
      <description></description>
      <id>528afd97-d8ad-414f-a9f2-df04210057c5</id>
      <masked>false</masked>
      <name>openingAccountPurpose</name>
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
	String jsPass =
	&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Root&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;enterpriseAccount&quot;: {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;id&quot;: {
            &quot;type&quot;: &quot;number&quot;
		},
        &quot;taxCode&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;companyName&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonName&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonPhone&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonEmail&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;supportStaffEmail&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;openingAccountPurpose&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;consent&quot;: {
            &quot;type&quot;: &quot;boolean&quot;
		},
        &quot;createdDate&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;lastModifiedDate&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;registrationAccount&quot;: {
            &quot;type&quot;: &quot;object&quot;,
            &quot;properties&quot;: {
                &quot;id&quot;: {
                    &quot;type&quot;: &quot;number&quot;
                },
                &quot;province&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;district&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;branch&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;accountNumber&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;selectedAccountType&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                }
            }
		},
        &quot;required&quot;: [
            &quot;id&quot;,
            &quot;taxCode&quot;,
            &quot;companyName&quot;,
            &quot;contactPersonName&quot;,
            &quot;contactPersonPhone&quot;,
            &quot;contactPersonEmail&quot;,
            &quot;supportStaffEmail&quot;,
            &quot;openingAccountPurpose&quot;,
            &quot;openingAccountPurpose&quot;,
            &quot;consent&quot;,
            &quot;registrationAccount&quot;
        ]
      }
		}
},
  &quot;required&quot;: [
    &quot;enterpriseAccount&quot;
  ]
};
&quot;&quot;&quot;
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsPass)
}
catch(Exception e) {
	throw &quot;API failed&quot;
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
