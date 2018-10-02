<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>main_Leads</name>
   <tag></tag>
   <elementGuidId>6a0db9cb-7cbc-4d22-b4a8-0dbcc271f0b3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>BASIC</key>
         <value>//main[@id = 'AjaxBody']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>main</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>AjaxBody</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>app-layout__content mdl-layout__content no-bg-img</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            






    
        
            
        
    
    
        Leads
    



    
        
            
                Private Leads 
                3
            
            
                Internet Leads 
                20
            
            
                Agent Leads 
                8
            
            
                Corporate Initiations 
                1
            
        
    

    
        
            
            Filters
        

        
            
                
                Agent/Contact Person
                
                    search
                
            
        

        
            
        


        
            
        

        
            Status
        

        
            
                    
                        
                        Abandonned
                    
                    
                        
                        Awaiting quote
                    
                    
                        
                        Draft
                    
                    
                        
                        Not Handled
                    
                    
                        
                        Pending PMS
                    
                    
                        
                        Pending
                    
                    
                        
                        Transferred
                    
            
            
                Apply 
            
        

        
            Clearclose
        
    
    





     Create Lead



    
        
            CREATED AGENTCONTACT PERSONMOVE DATEMODE OF TRANSPORTTYPE OF SERVICEVOL (m)ORIGIN COUNTRYDESTINATION COUNTRYALLOCATED TOSTATUSLeadStatusIDENQUIRY NO.26/9/2018 16:34Neetoo industriesTsiry RADIASON
        
            
                
            
        

    DS
        
        
            Vincent RODRIGUEZ
John SMITH
Moris NASSEAU
Not Allocated

            Moris NASSEAU
        
        
    
        
            Draft
        
    Draft1334126/9/2018 15:34Neetoo industriesEasy DEAL
        
            
                
            
        

    OSAfghanistanAfghanistan
        
        
            Vincent RODRIGUEZ
John SMITH
Moris NASSEAU
Not Allocated

            Moris NASSEAU
        
        
    
        
            Draft
        
    Draft1330826/9/2018 15:29Neetoo industriesEasy DEAL
        
            
                
            
        

    FSMauritiusMauritius
        
        
            Vincent RODRIGUEZ
John SMITH
Moris NASSEAU
Not Allocated

            Moris NASSEAU
        
        
    
        
            Draft
        
    Draft1330726/9/2018 14:15Neetoo industriesSufi RAZETTE27/9/2018
        
            
                
            
        

    FS4AlbaniaAzerbaijan
        
        
            Vincent RODRIGUEZ
John SMITH
Moris NASSEAU
Not Allocated

            Moris NASSEAU
        
        
    
        
            Not Handled
        
    Not Handled1328226/9/2018 11:50Indian oilyuna MOHIT
        
            
                
            
        
            
                
            
        

    FSMauritiusPalestinian Territories
        
        
            Vincent RODRIGUEZ
John SMITH
Moris NASSEAU
Not Allocated

            Moris NASSEAU
        
        
    
        
            Draft
        
    Draft13225...12345678...1 of 15 pages (73 items)FilterCREATEDAGENTCONTACT PERSONMOVE DATEMODE OF TRANSPORTTYPE OF SERVICEVOL (m)ORIGIN COUNTRYDESTINATION COUNTRYALLOCATED TOSTATUSLeadStatusIDENQUIRY NO.

            
                Rows:
                
            
        
    

$(function($){$(&quot;#AgentLeadsGrid&quot;).ejGrid({&quot;allowPaging&quot;:true,&quot;allowSorting&quot;:true,&quot;selectedRowIndices&quot;:[],&quot;_checkedRowIndices&quot;:[],&quot;dataSource&quot;:ej.DataManager({&quot;url&quot;:&quot;/Leads/GetGridData?qid=6ED7CE004600AF71140512554056140A1A100E6B86339B1642200607455064D1DB531F1EDE000502045C54510F0B0415&amp;status=AwaitingQuote, Draft, NotHandled, PendingPMS, Pending&amp;createDateRange=28/08/2018-27/09/2018&amp;moveDateRange=&amp;searchText=&amp;leadType=Agent&quot;,&quot;cachingPageSize&quot;:0,&quot;enableAJAXCache&quot;:false,&quot;adaptor&quot;:&quot;UrlAdaptor&quot;,&quot;headers&quot;:[]}),&quot;cssClass&quot;:&quot;&quot;,&quot;isResponsive&quot;:true,&quot;enableResponsiveRow&quot;:true,&quot;locale&quot;:&quot;en-GB&quot;,&quot;columns&quot;:[{&quot;field&quot;:&quot;FormattedCreationDate&quot;,&quot;headerText&quot;:&quot;CREATED&quot;,&quot;allowEditing&quot;:false,&quot;width&quot;:80,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;AgentName&quot;,&quot;headerText&quot;:&quot;AGENT&quot;,&quot;allowEditing&quot;:false,&quot;clipMode&quot;:&quot;ellipsis&quot;,&quot;width&quot;:80,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;AgentContactPerson&quot;,&quot;headerText&quot;:&quot;CONTACT PERSON&quot;,&quot;allowEditing&quot;:false,&quot;clipMode&quot;:&quot;ellipsis&quot;,&quot;width&quot;:85,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;FormattedMoveDate&quot;,&quot;headerText&quot;:&quot;MOVE DATE&quot;,&quot;allowEditing&quot;:false,&quot;width&quot;:80,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;ModeOfTransport&quot;,&quot;headerText&quot;:&quot;MODE OF TRANSPORT&quot;,&quot;allowEditing&quot;:false,&quot;template&quot;:&quot;#motIcons&quot;,&quot;width&quot;:100,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;TypeOfService&quot;,&quot;headerText&quot;:&quot;TYPE OF SERVICE&quot;,&quot;allowEditing&quot;:false,&quot;width&quot;:60,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;formattedVolume&quot;,&quot;headerText&quot;:&quot;VOL (m)&quot;,&quot;allowSorting&quot;:false,&quot;allowEditing&quot;:false,&quot;width&quot;:80,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;OriginCountry&quot;,&quot;headerText&quot;:&quot;ORIGIN COUNTRY&quot;,&quot;allowEditing&quot;:false,&quot;clipMode&quot;:&quot;ellipsis&quot;,&quot;width&quot;:85,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;DestinationCountry&quot;,&quot;headerText&quot;:&quot;DESTINATION COUNTRY&quot;,&quot;allowEditing&quot;:false,&quot;clipMode&quot;:&quot;ellipsis&quot;,&quot;width&quot;:85,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;AllocatedCoordinatorId&quot;,&quot;headerText&quot;:&quot;ALLOCATED TO&quot;,&quot;allowEditing&quot;:false,&quot;template&quot;:&quot;#coordinatorList&quot;,&quot;textAlign&quot;:&quot;center&quot;,&quot;width&quot;:130},{&quot;field&quot;:&quot;LeadStatus&quot;,&quot;headerText&quot;:&quot;STATUS&quot;,&quot;allowEditing&quot;:false,&quot;template&quot;:&quot;#gridStatus&quot;,&quot;textAlign&quot;:&quot;center&quot;,&quot;width&quot;:85},{&quot;field&quot;:&quot;Status&quot;,&quot;headerText&quot;:&quot;LeadStatus&quot;,&quot;allowEditing&quot;:false,&quot;textAlign&quot;:&quot;center&quot;,&quot;visible&quot;:false,&quot;width&quot;:85,&quot;type&quot;:&quot;string&quot;},{&quot;field&quot;:&quot;LeadId&quot;,&quot;headerText&quot;:&quot;ID&quot;,&quot;isPrimaryKey&quot;:true,&quot;visible&quot;:false,&quot;width&quot;:80,&quot;type&quot;:&quot;number&quot;},{&quot;field&quot;:&quot;EnquiryReference&quot;,&quot;headerText&quot;:&quot;ENQUIRY NO.&quot;,&quot;allowEditing&quot;:false,&quot;visible&quot;:false,&quot;width&quot;:80,&quot;type&quot;:&quot;string&quot;}],&quot;pageSettings&quot;:{&quot;pageSize&quot;:5},&quot;sortSettings&quot;:{&quot;sortedColumns&quot;:[{&quot;field&quot;:&quot;FormattedCreationDate&quot;,&quot;direction&quot;:&quot;descending&quot;}]},&quot;actionComplete&quot;:&quot;onLeadsGridActionCompleted&quot;,&quot;queryCellInfo&quot;:&quot;OnQueryCellInfo&quot;,&quot;recordClick&quot;:&quot;onAgentGridRecordClick&quot;});$(&quot;#pageSize&quot;).ejDropDownList({&quot;dataSource&quot;:ej.isJSON([5,10,15,20,25]),&quot;selectedItemIndex&quot;:0,&quot;selectedIndex&quot;:0,&quot;width&quot;:&quot;100%&quot;,&quot;locale&quot;:&quot;en-GB&quot;,&quot;change&quot;:&quot;sizeValChange&quot;});});




        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;AjaxBody&quot;)</value>
   </webElementProperties>
</WebElementEntity>
