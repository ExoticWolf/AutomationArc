<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_.stepper__navigation_assis</name>
   <tag></tag>
   <elementGuidId>d3362289-bff3-4e42-86ab-ae77df2da0e0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>stepper js-stepper</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    


    .stepper__navigation_assistance {
        max-width: 870px;
        justify-content: unset;
    }

        .stepper__navigation_assistance li:first-child {
            margin-right: 100px;
        }




    

                
                    
                        1
                        General Information
                    
                
                
                    
                        2
                        Scope of Service
                    
                
                
                    
                        3
                        Shipment information
                    
                
                
                    
                        4
                        Additional Information
                    
                
    

        
        

                    
                            * Mandatory fields
                        
 SEA










    
        
            m³
CFT
KG
LBS

            
            Volume/Weight *
            This field cannot be empty

            
            
                $(&quot;#seaUnitsVolumeWeightSelected&quot;).ejDropDownList({
                    enableFilterSearch: true,
                    value: &quot;V-2&quot;
                });
            
        

    



        
            
                
                
                
                Option 1 *
                This field cannot be empty
            
        
        
            
                
                
                
                Option 2
                This field cannot be empty
            
        
        
            
                
                
                
                Option 3
                This field cannot be empty
            
        


        
                





    
        20'
40' 
40' HC

        Type of container
        This field cannot be empty
    

    
        
    




    + Another container
                        
                        $(&quot;#sea_Option1_0&quot;).ejDropDownList({
                            enableFilterSearch: true,
                            value: &quot;3&quot;,
                        });

                        var optId = &quot;sea&quot; + &quot;_&quot; + &quot;Option1&quot;;
                        var optionValue = $(&quot;#&quot; + optId).val();

                        if (!optionValue) {
                            $(&quot;#sea_Option1_0&quot;).ejDropDownList({
                                enabled: false
                            });
                        }
                        
                
                





    
        20'
40' 
40' HC

        Type of container
        This field cannot be empty
    

    
        
    




    + Another container
                        
                        $(&quot;#sea_Option2_0&quot;).ejDropDownList({
                            enableFilterSearch: true,
                            value: &quot;4&quot;,
                        });

                        var optId = &quot;sea&quot; + &quot;_&quot; + &quot;Option2&quot;;
                        var optionValue = $(&quot;#&quot; + optId).val();

                        if (!optionValue) {
                            $(&quot;#sea_Option2_0&quot;).ejDropDownList({
                                enabled: false
                            });
                        }
                        
                
                





    
        20'
40' 
40' HC

        Type of container
        This field cannot be empty
    

    
        
    




    + Another container
                        
                        $(&quot;#sea_Option3_0&quot;).ejDropDownList({
                            enableFilterSearch: true,
                            value: &quot;4&quot;,
                        });

                        var optId = &quot;sea&quot; + &quot;_&quot; + &quot;Option3&quot;;
                        var optionValue = $(&quot;#&quot; + optId).val();

                        if (!optionValue) {
                            $(&quot;#sea_Option3_0&quot;).ejDropDownList({
                                enabled: false
                            });
                        }
                        
                
        

    


        
            
                
                    
                    
                    Allowance
                    
                
            
        

    

    
        
        
            + Additional specific items
        
        
        
            
                
                    
                        
                                    
                                        
                                            
                                        
                                            Vehicle
                                        
                                        
                                                    
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Brand
                                                                            
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Model
                                                                            
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            
                                                                            Year
                                                                        
                                                                    
                                                                

                                                                    
                                                                        
                                                                    

                                                    
                                                    
                                                        +
                                                        Another vehicle
                                                    

                                        
                                    
                                    
                                        
                                            
                                        
                                            Motorcycle
                                        
                                        
                                                    


                                                    

                                        
                                    
                                    
                                        
                                            
                                        
                                            Pet
                                        
                                        
                                                    
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Type
                                                                            
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Breed
                                                                            
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Name
                                                                            
                                                                        
                                                                    
                                                                

                                                                    
                                                                        
                                                                    

                                                    
                                                    
                                                        +
                                                        Another Pet
                                                    

                                        
                                    
                                    
                                        
                                            
                                        
                                            Heavy item
                                        
                                        
                                                    
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            Description
                                                                            
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            
                                                                            Approximate weight
                                                                        
                                                                    
                                                                
                                                                
                                                                    
                                                                    
                                                                    
                                                                        
                                                                            
KG
LBS

                                                                            Weight Unit
                                                                            
                                                                        
                                                                    
                                                                

                                                                    
                                                                        
                                                                    

                                                    
                                                    
                                                        +
                                                        Another heavy item
                                                    

                                        
                                    
                                    
                                        
                                            
                                        
                                            Grand Piano
                                        
                                        
                                                    


                                                    

                                        
                                    
                                    
                                        
                                            
                                        
                                            Upright piano
                                        
                                        
                                                    


                                                    

                                        
                                    

                        
                    
                
            
        
    





    

    
            
                Expected packaging period *

                
                    
                        
                        
                        
                        
                    

                    

                    
                        
                        
                        
                        
                    
                
            
            
                Wished delivery period *

                
                    
                        
                        
                        
                        
                    

                    

                    
                        
                        
                        
                        
                    
                
            
    
                    

            
                
                    
                        
                            Corporate Account
                            
                        
                    
                


Shipper Details


    
    



    .form-control.full-width.title{
        display: block;
    }



    
        
            
            
            
        

        

            
                Title *
                    
                        
                        Mr
                    
                    
                        
                        Mrs
                    
                    
                        
                        Miss
                    
                    
                        
                        Ms
                    
            
            Title cannot be null
        


    

    
        
            
                
                
                Name *
                Name cannot be empty
                
            
        

        
            
                
                
                Surname *
                Surname cannot be empty
                
            
        
    

    
        
            
                
                
                Email *
                Email cannot be empty
                
            
        

        
            
                United States+1United Kingdom+44Afghanistan (‫افغانستان‬‎)+93Albania (Shqipëri)+355Algeria (‫الجزائر‬‎)+213American Samoa+1684Andorra+376Angola+244Anguilla+1264Antigua and Barbuda+1268Argentina+54Armenia (Հայաստան)+374Aruba+297Australia+61Austria (Österreich)+43Azerbaijan (Azərbaycan)+994Bahamas+1242Bahrain (‫البحرين‬‎)+973Bangladesh (বাংলাদেশ)+880Barbados+1246Belarus (Беларусь)+375Belgium (België)+32Belize+501Benin (Bénin)+229Bermuda+1441Bhutan (འབྲུག)+975Bolivia+591Bosnia and Herzegovina (Босна и Херцеговина)+387Botswana+267Brazil (Brasil)+55British Indian Ocean Territory+246British Virgin Islands+1284Brunei+673Bulgaria (България)+359Burkina Faso+226Burundi (Uburundi)+257Cambodia (កម្ពុជា)+855Cameroon (Cameroun)+237Canada+1Cape Verde (Kabu Verdi)+238Caribbean Netherlands+599Cayman Islands+1345Central African Republic (République centrafricaine)+236Chad (Tchad)+235Chile+56China (中国)+86Christmas Island+61Cocos (Keeling) Islands+61Colombia+57Comoros (‫جزر القمر‬‎)+269Congo (DRC) (Jamhuri ya Kidemokrasia ya Kongo)+243Congo (Republic) (Congo-Brazzaville)+242Cook Islands+682Costa Rica+506Côte d’Ivoire+225Croatia (Hrvatska)+385Cuba+53Curaçao+599Cyprus (Κύπρος)+357Czech Republic (Česká republika)+420Denmark (Danmark)+45Djibouti+253Dominica+1767Dominican Republic (República Dominicana)+1Ecuador+593Egypt (‫مصر‬‎)+20El Salvador+503Equatorial Guinea (Guinea Ecuatorial)+240Eritrea+291Estonia (Eesti)+372Ethiopia+251Falkland Islands (Islas Malvinas)+500Faroe Islands (Føroyar)+298Fiji+679Finland (Suomi)+358France+33French Guiana (Guyane française)+594French Polynesia (Polynésie française)+689Gabon+241Gambia+220Georgia (საქართველო)+995Germany (Deutschland)+49Ghana (Gaana)+233Gibraltar+350Greece (Ελλάδα)+30Greenland (Kalaallit Nunaat)+299Grenada+1473Guadeloupe+590Guam+1671Guatemala+502Guernsey+44Guinea (Guinée)+224Guinea-Bissau (Guiné Bissau)+245Guyana+592Haiti+509Honduras+504Hong Kong (香港)+852Hungary (Magyarország)+36Iceland (Ísland)+354India (भारत)+91Indonesia+62Iran (‫ایران‬‎)+98Iraq (‫العراق‬‎)+964Ireland+353Isle of Man+44Israel (‫ישראל‬‎)+972Italy (Italia)+39Jamaica+1876Japan (日本)+81Jersey+44Jordan (‫الأردن‬‎)+962Kazakhstan (Казахстан)+7Kenya+254Kiribati+686Kosovo+383Kuwait (‫الكويت‬‎)+965Kyrgyzstan (Кыргызстан)+996Laos (ລາວ)+856Latvia (Latvija)+371Lebanon (‫لبنان‬‎)+961Lesotho+266Liberia+231Libya (‫ليبيا‬‎)+218Liechtenstein+423Lithuania (Lietuva)+370Luxembourg+352Macau (澳門)+853Macedonia (FYROM) (Македонија)+389Madagascar (Madagasikara)+261Malawi+265Malaysia+60Maldives+960Mali+223Malta+356Marshall Islands+692Martinique+596Mauritania (‫موريتانيا‬‎)+222Mauritius (Moris)+230Mayotte+262Mexico (México)+52Micronesia+691Moldova (Republica Moldova)+373Monaco+377Mongolia (Монгол)+976Montenegro (Crna Gora)+382Montserrat+1664Morocco (‫المغرب‬‎)+212Mozambique (Moçambique)+258Myanmar (Burma) (မြန်မာ)+95Namibia (Namibië)+264Nauru+674Nepal (नेपाल)+977Netherlands (Nederland)+31New Caledonia (Nouvelle-Calédonie)+687New Zealand+64Nicaragua+505Niger (Nijar)+227Nigeria+234Niue+683Norfolk Island+672North Korea (조선 민주주의 인민 공화국)+850Northern Mariana Islands+1670Norway (Norge)+47Oman (‫عُمان‬‎)+968Pakistan (‫پاکستان‬‎)+92Palau+680Palestine (‫فلسطين‬‎)+970Panama (Panamá)+507Papua New Guinea+675Paraguay+595Peru (Perú)+51Philippines+63Poland (Polska)+48Portugal+351Puerto Rico+1Qatar (‫قطر‬‎)+974Réunion (La Réunion)+262Romania (România)+40Russia (Россия)+7Rwanda+250Saint Barthélemy+590Saint Helena+290Saint Kitts and Nevis+1869Saint Lucia+1758Saint Martin (Saint-Martin (partie française))+590Saint Pierre and Miquelon (Saint-Pierre-et-Miquelon)+508Saint Vincent and the Grenadines+1784Samoa+685San Marino+378São Tomé and Príncipe (São Tomé e Príncipe)+239Saudi Arabia (‫المملكة العربية السعودية‬‎)+966Senegal (Sénégal)+221Serbia (Србија)+381Seychelles+248Sierra Leone+232Singapore+65Sint Maarten+1721Slovakia (Slovensko)+421Slovenia (Slovenija)+386Solomon Islands+677Somalia (Soomaaliya)+252South Africa+27South Korea (대한민국)+82South Sudan (‫جنوب السودان‬‎)+211Spain (España)+34Sri Lanka (ශ්‍රී ලංකාව)+94Sudan (‫السودان‬‎)+249Suriname+597Svalbard and Jan Mayen+47Swaziland+268Sweden (Sverige)+46Switzerland (Schweiz)+41Syria (‫سوريا‬‎)+963Taiwan (台灣)+886Tajikistan+992Tanzania+255Thailand (ไทย)+66Timor-Leste+670Togo+228Tokelau+690Tonga+676Trinidad and Tobago+1868Tunisia (‫تونس‬‎)+216Turkey (Türkiye)+90Turkmenistan+993Turks and Caicos Islands+1649Tuvalu+688U.S. Virgin Islands+1340Uganda+256Ukraine (Україна)+380United Arab Emirates (‫الإمارات العربية المتحدة‬‎)+971United Kingdom+44United States+1Uruguay+598Uzbekistan (Oʻzbekiston)+998Vanuatu+678Vatican City (Città del Vaticano)+39Venezuela+58Vietnam (Việt Nam)+84Wallis and Futuna (Wallis-et-Futuna)+681Western Sahara (‫الصحراء الغربية‬‎)+212Yemen (‫اليمن‬‎)+967Zambia+260Zimbabwe+263Åland Islands+358
                
                Mobile Number *
                Mobile number cannot be empty
                
                A valid phone number is required.
            
        
    

         

        
            
                
                Comments
            
        
    






    $('#formCreateContact input').click(function () {
        var name = $(this).attr(&quot;name&quot;);
        $(&quot;#&quot; + name + &quot;_validationMessage&quot;).css(&quot;display&quot;, &quot;none&quot;);
    });

    $('#formCreateContact input').change(function () {
        var name = $(this).attr(&quot;name&quot;);
        $(&quot;#&quot; + name + &quot;_validationMessage&quot;).css(&quot;display&quot;, &quot;none&quot;);

        if ($(this).attr(&quot;id&quot;) == &quot;Email&quot;) {
            $(&quot;#&quot; + name + &quot;_validationMessage&quot;).html(&quot;This field cannot be empty&quot;);
        }
    });

    $('#formCreateContact select').change(function () {
        var name = $(this).attr(&quot;name&quot;);
        $(&quot;#&quot; + name + &quot;_validationMessage&quot;).css(&quot;display&quot;, &quot;none&quot;);
    });
    function saveShipper() {
        if (ValidateContactForm() === true) {
            //alert(&quot;true&quot;);
            
        }
    }


                
                    
                        
                            Previous
                            Scope of Service
                        

                        
                    

                    
                        
                            Next
                            Additional Information
                        
                        
                    
                
            
        

//&lt;![CDATA[
if (!window.mvcClientValidationMetadata) { window.mvcClientValidationMetadata = []; }
window.mvcClientValidationMetadata.push({&quot;Fields&quot;:[{&quot;FieldName&quot;:&quot;ListUnitsVolumeWeight&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;ListUnitsVolumeWeight_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;OptionsContainer[Option1].OptionVolumeWeight&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;sea_Option1_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;OptionsContainer[Option2].OptionVolumeWeight&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;sea_Option2_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;OptionsContainer[Option3].OptionVolumeWeight&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;sea_Option3_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;Allowance&quot;,&quot;ReplaceValidationMessageContents&quot;:true,&quot;ValidationMessageId&quot;:&quot;Allowance_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;TitleId&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;TitleId_validationMessage&quot;,&quot;ValidationRules&quot;:[{&quot;ErrorMessage&quot;:&quot;The field TitleId must be a number.&quot;,&quot;ValidationParameters&quot;:{},&quot;ValidationType&quot;:&quot;number&quot;}]},{&quot;FieldName&quot;:&quot;Name&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;Name_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;Surname&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;Surname_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;Email&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;Email_validationMessage&quot;,&quot;ValidationRules&quot;:[]},{&quot;FieldName&quot;:&quot;MobileNumber&quot;,&quot;ReplaceValidationMessageContents&quot;:false,&quot;ValidationMessageId&quot;:&quot;MobileNumber_validationMessage&quot;,&quot;ValidationRules&quot;:[]}],&quot;FormId&quot;:&quot;frmShipmentInfo&quot;,&quot;ReplaceValidationSummary&quot;:false});
//]]>
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;AjaxBody&quot;)/div[@class=&quot;stepper js-stepper&quot;]</value>
   </webElementProperties>
</WebElementEntity>
