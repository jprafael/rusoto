#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;

        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
    
use serde_json;
        use rusoto_core::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;
pub type ActionOnFailure = String;
#[doc="<p>Input to an AddInstanceGroups call.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddInstanceGroupsInput {
                #[doc="<p>Instance groups to add.</p>"]
#[serde(rename="InstanceGroups")]
pub instance_groups: InstanceGroupConfigList,
#[doc="<p>Job flow in which to add the instance groups.</p>"]
#[serde(rename="JobFlowId")]
pub job_flow_id: XmlStringMaxLen256,
            }
            
#[doc="<p>Output from an AddInstanceGroups call.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AddInstanceGroupsOutput {
                #[doc="<p>Instance group IDs of the newly created instance groups.</p>"]
#[serde(rename="InstanceGroupIds")]
pub instance_group_ids: Option<InstanceGroupIdsList>,
#[doc="<p>The job flow ID in which the instance groups are added.</p>"]
#[serde(rename="JobFlowId")]
pub job_flow_id: Option<XmlStringMaxLen256>,
            }
            
#[doc="<p> The input argument to the <a>AddJobFlowSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddJobFlowStepsInput {
                #[doc="<p>A string that uniquely identifies the job flow. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>ListClusters</a>. </p>"]
#[serde(rename="JobFlowId")]
pub job_flow_id: XmlStringMaxLen256,
#[doc="<p> A list of <a>StepConfig</a> to be executed by the job flow. </p>"]
#[serde(rename="Steps")]
pub steps: StepConfigList,
            }
            
#[doc="<p> The output for the <a>AddJobFlowSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AddJobFlowStepsOutput {
                #[doc="<p>The identifiers of the list of steps added to the job flow.</p>"]
#[serde(rename="StepIds")]
pub step_ids: Option<StepIdsList>,
            }
            
#[doc="<p>This input identifies a cluster and a list of tags to attach.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddTagsInput {
                #[doc="<p>The Amazon EMR resource identifier to which tags will be added. This value must be a cluster identifier.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: ResourceId,
#[doc="<p>A list of tags to associate with a cluster and propagate to EC2 instances. Tags are user-defined key/value pairs that consist of a required key string with a maximum of 128 characters, and an optional value string with a maximum of 256 characters.</p>"]
#[serde(rename="Tags")]
pub tags: TagList,
            }
            
#[doc="<p>This output indicates the result of adding tags to a resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AddTagsOutput;
            
pub type AdjustmentType = String;
#[doc="<p>An application is any Amazon or third-party software that you can add to the cluster. This structure contains a list of strings that indicates the software to use with the cluster and accepts a user argument list. Amazon EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action argument. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-mapr.html\">Launch a Job Flow on the MapR Distribution for Hadoop</a>. Currently supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the job flow using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the job flow using MapR M5 Edition.</p> </li> <li> <p>\"mapr\" with the user arguments specifying \"--edition,m3\" or \"--edition,m5\" - launch the job flow using MapR M3 or M5 Edition, respectively.</p> </li> </ul> <note> <p>In Amazon EMR releases 4.0 and greater, the only accepted parameter is the application name. To pass arguments to applications, you supply a configuration for each application.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Application {
                #[doc="<p>This option is for advanced users only. This is meta information about third-party applications that third-party vendors use for testing purposes.</p>"]
#[serde(rename="AdditionalInfo")]
pub additional_info: Option<StringMap>,
#[doc="<p>Arguments for Amazon EMR to pass to the application.</p>"]
#[serde(rename="Args")]
pub args: Option<StringList>,
#[doc="<p>The name of the application.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>The version of the application.</p>"]
#[serde(rename="Version")]
pub version: Option<String>,
            }
            
pub type ApplicationList = Vec<Application>;
#[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. An automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AutoScalingPolicy {
                #[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>"]
#[serde(rename="Constraints")]
pub constraints: ScalingConstraints,
#[doc="<p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>"]
#[serde(rename="Rules")]
pub rules: ScalingRuleList,
            }
            
#[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutoScalingPolicyDescription {
                #[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>"]
#[serde(rename="Constraints")]
pub constraints: Option<ScalingConstraints>,
#[doc="<p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>"]
#[serde(rename="Rules")]
pub rules: Option<ScalingRuleList>,
#[doc="<p>The status of an automatic scaling policy. </p>"]
#[serde(rename="Status")]
pub status: Option<AutoScalingPolicyStatus>,
            }
            
pub type AutoScalingPolicyState = String;
#[doc="<p>The reason for an <a>AutoScalingPolicyStatus</a> change.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutoScalingPolicyStateChangeReason {
                #[doc="<p>The code indicating the reason for the change in status.<code>USER_REQUEST</code> indicates that the scaling policy status was changed by a user. <code>PROVISION_FAILURE</code> indicates that the status change was because the policy failed to provision. <code>CLEANUP_FAILURE</code> indicates something unclean happened.--&gt;</p>"]
#[serde(rename="Code")]
pub code: Option<AutoScalingPolicyStateChangeReasonCode>,
#[doc="<p>A friendly, more verbose message that accompanies an automatic scaling policy state change.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
            }
            
pub type AutoScalingPolicyStateChangeReasonCode = String;
#[doc="<p>The status of an automatic scaling policy. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutoScalingPolicyStatus {
                #[doc="<p></p>"]
#[serde(rename="State")]
pub state: Option<AutoScalingPolicyState>,
#[doc="<p>The reason for a change in status.</p>"]
#[serde(rename="StateChangeReason")]
pub state_change_reason: Option<AutoScalingPolicyStateChangeReason>,
            }
            
pub type Boolean = bool;
pub type BooleanObject = bool;
#[doc="<p>Configuration of a bootstrap action.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct BootstrapActionConfig {
                #[doc="<p>The name of the bootstrap action.</p>"]
#[serde(rename="Name")]
pub name: XmlStringMaxLen256,
#[doc="<p>The script run by the bootstrap action.</p>"]
#[serde(rename="ScriptBootstrapAction")]
pub script_bootstrap_action: ScriptBootstrapActionConfig,
            }
            
pub type BootstrapActionConfigList = Vec<BootstrapActionConfig>;
#[doc="<p>Reports the configuration of a bootstrap action in a job flow.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BootstrapActionDetail {
                #[doc="<p>A description of the bootstrap action.</p>"]
#[serde(rename="BootstrapActionConfig")]
pub bootstrap_action_config: Option<BootstrapActionConfig>,
            }
            
pub type BootstrapActionDetailList = Vec<BootstrapActionDetail>;
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CancelStepsInfo {
                #[serde(rename="Reason")]
pub reason: Option<String>,
#[serde(rename="Status")]
pub status: Option<CancelStepsRequestStatus>,
#[serde(rename="StepId")]
pub step_id: Option<StepId>,
            }
            
pub type CancelStepsInfoList = Vec<CancelStepsInfo>;
#[doc="<p>The input argument to the <a>CancelSteps</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CancelStepsInput {
                #[doc="<p>The <code>ClusterID</code> for which specified steps will be canceled. Use <a>RunJobFlow</a> and <a>ListClusters</a> to get ClusterIDs. </p>"]
#[serde(rename="ClusterId")]
pub cluster_id: Option<XmlStringMaxLen256>,
#[doc="<p>The list of <code>StepIDs</code> to cancel. Use <a>ListSteps</a> to get steps and their states for the specified cluster.</p>"]
#[serde(rename="StepIds")]
pub step_ids: Option<StepIdsList>,
            }
            
#[doc="<p> The output for the <a>CancelSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CancelStepsOutput {
                #[doc="<p>A list of <a>CancelStepsInfo</a>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>"]
#[serde(rename="CancelStepsInfoList")]
pub cancel_steps_info_list: Option<CancelStepsInfoList>,
            }
            
pub type CancelStepsRequestStatus = String;
#[doc="<p>The definition of a CloudWatch metric alarm, which determines when an automatic scaling activity is triggered. When the defined alarm conditions are satisfied, scaling activity begins.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct CloudWatchAlarmDefinition {
                #[doc="<p>Determines how the metric specified by <code>MetricName</code> is compared to the value specified by <code>Threshold</code>.</p>"]
#[serde(rename="ComparisonOperator")]
pub comparison_operator: ComparisonOperator,
#[doc="<p>A CloudWatch metric dimension.</p>"]
#[serde(rename="Dimensions")]
pub dimensions: Option<MetricDimensionList>,
#[doc="<p>The number of periods, expressed in seconds using <code>Period</code>, during which the alarm condition must exist before the alarm triggers automatic scaling activity. The default value is <code>1</code>.</p>"]
#[serde(rename="EvaluationPeriods")]
pub evaluation_periods: Option<Integer>,
#[doc="<p>The name of the CloudWatch metric that is watched to determine an alarm condition.</p>"]
#[serde(rename="MetricName")]
pub metric_name: String,
#[doc="<p>The namespace for the CloudWatch metric. The default is <code>AWS/ElasticMapReduce</code>.</p>"]
#[serde(rename="Namespace")]
pub namespace: Option<String>,
#[doc="<p>The period, in seconds, over which the statistic is applied. EMR CloudWatch metrics are emitted every five minutes (300 seconds), so if an EMR CloudWatch metric is specified, specify <code>300</code>.</p>"]
#[serde(rename="Period")]
pub period: Integer,
#[doc="<p>The statistic to apply to the metric associated with the alarm. The default is <code>AVERAGE</code>.</p>"]
#[serde(rename="Statistic")]
pub statistic: Option<Statistic>,
#[doc="<p>The value against which the specified statistic is compared.</p>"]
#[serde(rename="Threshold")]
pub threshold: NonNegativeDouble,
#[doc="<p>The unit of measure associated with the CloudWatch metric being watched. The value specified for <code>Unit</code> must correspond to the units specified in the CloudWatch metric.</p>"]
#[serde(rename="Unit")]
pub unit: Option<Unit>,
            }
            
#[doc="<p>The detailed description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Cluster {
                #[doc="<p>The applications installed on this cluster.</p>"]
#[serde(rename="Applications")]
pub applications: Option<ApplicationList>,
#[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>"]
#[serde(rename="AutoScalingRole")]
pub auto_scaling_role: Option<XmlString>,
#[doc="<p>Specifies whether the cluster should terminate after completing all steps.</p>"]
#[serde(rename="AutoTerminate")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub auto_terminate: Option<Boolean>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of Configurations supplied to the EMR cluster.</p>"]
#[serde(rename="Configurations")]
pub configurations: Option<ConfigurationList>,
#[doc="<p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>"]
#[serde(rename="Ec2InstanceAttributes")]
pub ec_2_instance_attributes: Option<Ec2InstanceAttributes>,
#[doc="<p>The unique identifier for the cluster.</p>"]
#[serde(rename="Id")]
pub id: Option<ClusterId>,
#[doc="<p>The path to the Amazon S3 location where logs for this cluster are stored.</p>"]
#[serde(rename="LogUri")]
pub log_uri: Option<String>,
#[doc="<p>The public DNS name of the master EC2 instance.</p>"]
#[serde(rename="MasterPublicDnsName")]
pub master_public_dns_name: Option<String>,
#[doc="<p>The name of the cluster.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>An approximation of the cost of the job flow, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
#[serde(rename="NormalizedInstanceHours")]
pub normalized_instance_hours: Option<Integer>,
#[doc="<p>The release label for the Amazon EMR release. For Amazon EMR 3.x and 2.x AMIs, use amiVersion instead instead of ReleaseLabel.</p>"]
#[serde(rename="ReleaseLabel")]
pub release_label: Option<String>,
#[doc="<p>The AMI version requested for this cluster.</p>"]
#[serde(rename="RequestedAmiVersion")]
pub requested_ami_version: Option<String>,
#[doc="<p>The AMI version running on this cluster.</p>"]
#[serde(rename="RunningAmiVersion")]
pub running_ami_version: Option<String>,
#[doc="<p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> is available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
#[serde(rename="ScaleDownBehavior")]
pub scale_down_behavior: Option<ScaleDownBehavior>,
#[doc="<p>The name of the security configuration applied to the cluster.</p>"]
#[serde(rename="SecurityConfiguration")]
pub security_configuration: Option<XmlString>,
#[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<String>,
#[doc="<p>The current status details about the cluster.</p>"]
#[serde(rename="Status")]
pub status: Option<ClusterStatus>,
#[doc="<p>A list of tags associated with a cluster.</p>"]
#[serde(rename="Tags")]
pub tags: Option<TagList>,
#[doc="<p>Indicates whether Amazon EMR will lock the cluster to prevent the EC2 instances from being terminated by an API call or user intervention, or in the event of a cluster error.</p>"]
#[serde(rename="TerminationProtected")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub termination_protected: Option<Boolean>,
#[doc="<p>Indicates whether the job flow is visible to all IAM users of the AWS account associated with the job flow. If this value is set to <code>true</code>, all IAM users of that AWS account can view and manage the job flow if they have the proper policy permissions set. If this value is <code>false</code>, only the IAM user that created the cluster can view and manage it. This value can be changed using the <a>SetVisibleToAllUsers</a> action.</p>"]
#[serde(rename="VisibleToAllUsers")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub visible_to_all_users: Option<Boolean>,
            }
            
pub type ClusterId = String;
pub type ClusterState = String;
#[doc="<p>The reason that the cluster changed to its current state.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ClusterStateChangeReason {
                #[doc="<p>The programmatic code for the state change reason.</p>"]
#[serde(rename="Code")]
pub code: Option<ClusterStateChangeReasonCode>,
#[doc="<p>The descriptive message for the state change reason.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
            }
            
pub type ClusterStateChangeReasonCode = String;
pub type ClusterStateList = Vec<ClusterState>;
#[doc="<p>The detailed status of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ClusterStatus {
                #[doc="<p>The current state of the cluster.</p>"]
#[serde(rename="State")]
pub state: Option<ClusterState>,
#[doc="<p>The reason for the cluster status change.</p>"]
#[serde(rename="StateChangeReason")]
pub state_change_reason: Option<ClusterStateChangeReason>,
#[doc="<p>A timeline that represents the status of a cluster over the lifetime of the cluster.</p>"]
#[serde(rename="Timeline")]
pub timeline: Option<ClusterTimeline>,
            }
            
#[doc="<p>The summary description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ClusterSummary {
                #[doc="<p>The unique identifier for the cluster.</p>"]
#[serde(rename="Id")]
pub id: Option<ClusterId>,
#[doc="<p>The name of the cluster.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>An approximation of the cost of the job flow, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
#[serde(rename="NormalizedInstanceHours")]
pub normalized_instance_hours: Option<Integer>,
#[doc="<p>The details about the current status of the cluster.</p>"]
#[serde(rename="Status")]
pub status: Option<ClusterStatus>,
            }
            
pub type ClusterSummaryList = Vec<ClusterSummary>;
#[doc="<p>Represents the timeline of the cluster's lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ClusterTimeline {
                #[doc="<p>The creation date and time of the cluster.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The date and time when the cluster was terminated.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>The date and time when the cluster was ready to execute steps.</p>"]
#[serde(rename="ReadyDateTime")]
pub ready_date_time: Option<Date>,
            }
            
#[doc="<p>An entity describing an executable that runs on a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Command {
                #[doc="<p>Arguments for Amazon EMR to pass to the command for execution.</p>"]
#[serde(rename="Args")]
pub args: Option<StringList>,
#[doc="<p>The name of the command.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>The Amazon S3 location of the command script.</p>"]
#[serde(rename="ScriptPath")]
pub script_path: Option<String>,
            }
            
pub type CommandList = Vec<Command>;
pub type ComparisonOperator = String;
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>Specifies a hardware and software configuration of the EMR cluster. This includes configurations for applications and software bundled with Amazon EMR. The Configuration object is a JSON object which is defined by a classification and a set of properties. Configurations can be nested, so a configuration may have its own Configuration objects listed.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Configuration {
                #[doc="<p>The classification of a configuration. For more information see, <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/API/EmrConfigurations.html\">Amazon EMR Configurations</a>. </p>"]
#[serde(rename="Classification")]
pub classification: Option<String>,
#[doc="<p>A list of configurations you apply to this configuration object.</p>"]
#[serde(rename="Configurations")]
pub configurations: Option<ConfigurationList>,
#[doc="<p>A set of properties supplied to the Configuration object.</p>"]
#[serde(rename="Properties")]
pub properties: Option<StringMap>,
            }
            
pub type ConfigurationList = Vec<Configuration>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateSecurityConfigurationInput {
                #[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: XmlString,
#[doc="<p>The security configuration details in JSON format.</p>"]
#[serde(rename="SecurityConfiguration")]
pub security_configuration: String,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateSecurityConfigurationOutput {
                #[doc="<p>The date and time the security configuration was created.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Date,
#[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: XmlString,
            }
            
pub type Date = f64;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteSecurityConfigurationInput {
                #[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: XmlString,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteSecurityConfigurationOutput;
            
#[doc="<p>This input determines which cluster to describe.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeClusterInput {
                #[doc="<p>The identifier of the cluster to describe.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
            }
            
#[doc="<p>This output contains the description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeClusterOutput {
                #[doc="<p>This output contains the details for the requested cluster.</p>"]
#[serde(rename="Cluster")]
pub cluster: Option<Cluster>,
            }
            
#[doc="<p> The input for the <a>DescribeJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeJobFlowsInput {
                #[doc="<p>Return only job flows created after this date and time.</p>"]
#[serde(rename="CreatedAfter")]
pub created_after: Option<Date>,
#[doc="<p>Return only job flows created before this date and time.</p>"]
#[serde(rename="CreatedBefore")]
pub created_before: Option<Date>,
#[doc="<p>Return only job flows whose job flow ID is contained in this list.</p>"]
#[serde(rename="JobFlowIds")]
pub job_flow_ids: Option<XmlStringList>,
#[doc="<p>Return only job flows whose state is contained in this list.</p>"]
#[serde(rename="JobFlowStates")]
pub job_flow_states: Option<JobFlowExecutionStateList>,
            }
            
#[doc="<p> The output for the <a>DescribeJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeJobFlowsOutput {
                #[doc="<p>A list of job flows matching the parameters supplied.</p>"]
#[serde(rename="JobFlows")]
pub job_flows: Option<JobFlowDetailList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeSecurityConfigurationInput {
                #[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: XmlString,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeSecurityConfigurationOutput {
                #[doc="<p>The date and time the security configuration was created</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: Option<XmlString>,
#[doc="<p>The security configuration details in JSON format.</p>"]
#[serde(rename="SecurityConfiguration")]
pub security_configuration: Option<String>,
            }
            
#[doc="<p>This input determines which step to describe.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeStepInput {
                #[doc="<p>The identifier of the cluster with steps to describe.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>The identifier of the step to describe.</p>"]
#[serde(rename="StepId")]
pub step_id: StepId,
            }
            
#[doc="<p>This output contains the description of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeStepOutput {
                #[doc="<p>The step details for the requested step identifier.</p>"]
#[serde(rename="Step")]
pub step: Option<Step>,
            }
            
pub type EC2InstanceIdsList = Vec<InstanceId>;
pub type EC2InstanceIdsToTerminateList = Vec<InstanceId>;
#[doc="<p>Configuration of requested EBS block device associated with the instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct EbsBlockDevice {
                #[doc="<p>The device name that is exposed to the instance, such as /dev/sdh.</p>"]
#[serde(rename="Device")]
pub device: Option<String>,
#[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
#[serde(rename="VolumeSpecification")]
pub volume_specification: Option<VolumeSpecification>,
            }
            
#[doc="<p>Configuration of requested EBS block device associated with the instance group with count of volumes that will be associated to every instance.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct EbsBlockDeviceConfig {
                #[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
#[serde(rename="VolumeSpecification")]
pub volume_specification: VolumeSpecification,
#[doc="<p>Number of EBS volumes with a specific volume configuration that will be associated with every instance in the instance group</p>"]
#[serde(rename="VolumesPerInstance")]
pub volumes_per_instance: Option<Integer>,
            }
            
pub type EbsBlockDeviceConfigList = Vec<EbsBlockDeviceConfig>;
pub type EbsBlockDeviceList = Vec<EbsBlockDevice>;
#[doc="<p>The Amazon EBS configuration of a cluster instance.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct EbsConfiguration {
                #[doc="<p>An array of Amazon EBS volume specifications attached to a cluster instance.</p>"]
#[serde(rename="EbsBlockDeviceConfigs")]
pub ebs_block_device_configs: Option<EbsBlockDeviceConfigList>,
#[doc="<p>Indicates whether an Amazon EBS volume is EBS-optimized.</p>"]
#[serde(rename="EbsOptimized")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub ebs_optimized: Option<BooleanObject>,
            }
            
#[doc="<p>EBS block device that's attached to an EC2 instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct EbsVolume {
                #[doc="<p>The device name that is exposed to the instance, such as /dev/sdh.</p>"]
#[serde(rename="Device")]
pub device: Option<String>,
#[doc="<p>The volume identifier of the EBS volume.</p>"]
#[serde(rename="VolumeId")]
pub volume_id: Option<String>,
            }
            
pub type EbsVolumeList = Vec<EbsVolume>;
#[doc="<p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Ec2InstanceAttributes {
                #[doc="<p>A list of additional Amazon EC2 security group IDs for the master node.</p>"]
#[serde(rename="AdditionalMasterSecurityGroups")]
pub additional_master_security_groups: Option<StringList>,
#[doc="<p>A list of additional Amazon EC2 security group IDs for the slave nodes.</p>"]
#[serde(rename="AdditionalSlaveSecurityGroups")]
pub additional_slave_security_groups: Option<StringList>,
#[doc="<p>The Availability Zone in which the cluster will run.</p>"]
#[serde(rename="Ec2AvailabilityZone")]
pub ec_2_availability_zone: Option<String>,
#[doc="<p>The name of the Amazon EC2 key pair to use when connecting with SSH into the master node as a user named \"hadoop\".</p>"]
#[serde(rename="Ec2KeyName")]
pub ec_2_key_name: Option<String>,
#[doc="<p>To launch the job flow in Amazon VPC, set this parameter to the identifier of the Amazon VPC subnet where you want the job flow to launch. If you do not specify this value, the job flow is launched in the normal AWS cloud, outside of a VPC.</p> <p>Amazon VPC currently does not support cluster compute quadruple extra large (cc1.4xlarge) instances. Thus, you cannot specify the cc1.4xlarge instance type for nodes of a job flow launched in a VPC.</p>"]
#[serde(rename="Ec2SubnetId")]
pub ec_2_subnet_id: Option<String>,
#[doc="<p>The identifier of the Amazon EC2 security group for the master node.</p>"]
#[serde(rename="EmrManagedMasterSecurityGroup")]
pub emr_managed_master_security_group: Option<String>,
#[doc="<p>The identifier of the Amazon EC2 security group for the slave nodes.</p>"]
#[serde(rename="EmrManagedSlaveSecurityGroup")]
pub emr_managed_slave_security_group: Option<String>,
#[doc="<p>The IAM role that was specified when the job flow was launched. The EC2 instances of the job flow assume this role.</p>"]
#[serde(rename="IamInstanceProfile")]
pub iam_instance_profile: Option<String>,
#[doc="<p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>"]
#[serde(rename="ServiceAccessSecurityGroup")]
pub service_access_security_group: Option<String>,
            }
            
pub type ErrorCode = String;
pub type ErrorMessage = String;
#[doc="<p>The details of the step failure. The service attempts to detect the root cause for many common failures.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct FailureDetails {
                #[doc="<p>The path to the log file where the step failure root cause was originally recorded.</p>"]
#[serde(rename="LogFile")]
pub log_file: Option<String>,
#[doc="<p>The descriptive message including the error the EMR service has identified as the cause of step failure. This is text from an error log that describes the root cause of the failure.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
#[doc="<p>The reason for the step failure. In the case where the service cannot successfully determine the root cause of the failure, it returns \"Unknown Error\" as a reason.</p>"]
#[serde(rename="Reason")]
pub reason: Option<String>,
            }
            
#[doc="<p>A job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct HadoopJarStepConfig {
                #[doc="<p>A list of command line arguments passed to the JAR file's main function when executed.</p>"]
#[serde(rename="Args")]
pub args: Option<XmlStringList>,
#[doc="<p>A path to a JAR file run during the step.</p>"]
#[serde(rename="Jar")]
pub jar: XmlString,
#[doc="<p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a Main-Class in its manifest file.</p>"]
#[serde(rename="MainClass")]
pub main_class: Option<XmlString>,
#[doc="<p>A list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>"]
#[serde(rename="Properties")]
pub properties: Option<KeyValueList>,
            }
            
#[doc="<p>A cluster step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct HadoopStepConfig {
                #[doc="<p>The list of command line arguments to pass to the JAR file's main function for execution.</p>"]
#[serde(rename="Args")]
pub args: Option<StringList>,
#[doc="<p>The path to the JAR file that runs during the step.</p>"]
#[serde(rename="Jar")]
pub jar: Option<String>,
#[doc="<p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a main class in its manifest file.</p>"]
#[serde(rename="MainClass")]
pub main_class: Option<String>,
#[doc="<p>The list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>"]
#[serde(rename="Properties")]
pub properties: Option<StringMap>,
            }
            
#[doc="<p>Represents an EC2 instance provisioned as part of cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Instance {
                #[doc="<p>The list of EBS volumes that are attached to this instance.</p>"]
#[serde(rename="EbsVolumes")]
pub ebs_volumes: Option<EbsVolumeList>,
#[doc="<p>The unique identifier of the instance in Amazon EC2.</p>"]
#[serde(rename="Ec2InstanceId")]
pub ec_2_instance_id: Option<InstanceId>,
#[doc="<p>The unique identifier for the instance in Amazon EMR.</p>"]
#[serde(rename="Id")]
pub id: Option<InstanceId>,
#[doc="<p>The identifier of the instance group to which this instance belongs.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: Option<String>,
#[doc="<p>The private DNS name of the instance.</p>"]
#[serde(rename="PrivateDnsName")]
pub private_dns_name: Option<String>,
#[doc="<p>The private IP address of the instance.</p>"]
#[serde(rename="PrivateIpAddress")]
pub private_ip_address: Option<String>,
#[doc="<p>The public DNS name of the instance.</p>"]
#[serde(rename="PublicDnsName")]
pub public_dns_name: Option<String>,
#[doc="<p>The public IP address of the instance.</p>"]
#[serde(rename="PublicIpAddress")]
pub public_ip_address: Option<String>,
#[doc="<p>The current status of the instance.</p>"]
#[serde(rename="Status")]
pub status: Option<InstanceStatus>,
            }
            
#[doc="<p>This entity represents an instance group, which is a group of instances that have common purpose. For example, CORE instance group is used for HDFS.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceGroup {
                #[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See PutAutoScalingPolicy.</p>"]
#[serde(rename="AutoScalingPolicy")]
pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
#[doc="<p>The bid price for each EC2 instance in the instance group when launching nodes as Spot Instances, expressed in USD.</p>"]
#[serde(rename="BidPrice")]
pub bid_price: Option<String>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p>"]
#[serde(rename="Configurations")]
pub configurations: Option<ConfigurationList>,
#[doc="<p>The EBS block devices that are mapped to this instance group.</p>"]
#[serde(rename="EbsBlockDevices")]
pub ebs_block_devices: Option<EbsBlockDeviceList>,
#[doc="<p>If the instance group is EBS-optimized. An Amazon EBS-optimized instance uses an optimized configuration stack and provides additional, dedicated capacity for Amazon EBS I/O.</p>"]
#[serde(rename="EbsOptimized")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub ebs_optimized: Option<BooleanObject>,
#[doc="<p>The identifier of the instance group.</p>"]
#[serde(rename="Id")]
pub id: Option<InstanceGroupId>,
#[doc="<p>The type of the instance group. Valid values are MASTER, CORE or TASK.</p>"]
#[serde(rename="InstanceGroupType")]
pub instance_group_type: Option<InstanceGroupType>,
#[doc="<p>The EC2 instance type for all instances in the instance group.</p>"]
#[serde(rename="InstanceType")]
pub instance_type: Option<InstanceType>,
#[doc="<p>The marketplace to provision instances for this group. Valid values are ON_DEMAND or SPOT.</p>"]
#[serde(rename="Market")]
pub market: Option<MarketType>,
#[doc="<p>The name of the instance group.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>The target number of instances for the instance group.</p>"]
#[serde(rename="RequestedInstanceCount")]
pub requested_instance_count: Option<Integer>,
#[doc="<p>The number of instances currently running in this instance group.</p>"]
#[serde(rename="RunningInstanceCount")]
pub running_instance_count: Option<Integer>,
#[doc="<p>Policy for customizing shrink operations.</p>"]
#[serde(rename="ShrinkPolicy")]
pub shrink_policy: Option<ShrinkPolicy>,
#[doc="<p>The current status of the instance group.</p>"]
#[serde(rename="Status")]
pub status: Option<InstanceGroupStatus>,
            }
            
#[doc="<p>Configuration defining a new instance group.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InstanceGroupConfig {
                #[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
#[serde(rename="AutoScalingPolicy")]
pub auto_scaling_policy: Option<AutoScalingPolicy>,
#[doc="<p>Bid price for each EC2 instance in the instance group when launching nodes as Spot Instances, expressed in USD.</p>"]
#[serde(rename="BidPrice")]
pub bid_price: Option<XmlStringMaxLen256>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p>"]
#[serde(rename="Configurations")]
pub configurations: Option<ConfigurationList>,
#[doc="<p>EBS configurations that will be attached to each EC2 instance in the instance group.</p>"]
#[serde(rename="EbsConfiguration")]
pub ebs_configuration: Option<EbsConfiguration>,
#[doc="<p>Target number of instances for the instance group.</p>"]
#[serde(rename="InstanceCount")]
pub instance_count: Integer,
#[doc="<p>The role of the instance group in the cluster.</p>"]
#[serde(rename="InstanceRole")]
pub instance_role: InstanceRoleType,
#[doc="<p>The EC2 instance type for all instances in the instance group.</p>"]
#[serde(rename="InstanceType")]
pub instance_type: InstanceType,
#[doc="<p>Market type of the EC2 instances used to create a cluster node.</p>"]
#[serde(rename="Market")]
pub market: Option<MarketType>,
#[doc="<p>Friendly name given to the instance group.</p>"]
#[serde(rename="Name")]
pub name: Option<XmlStringMaxLen256>,
            }
            
pub type InstanceGroupConfigList = Vec<InstanceGroupConfig>;
#[doc="<p>Detailed information about an instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceGroupDetail {
                #[doc="<p>Bid price for EC2 Instances when launching nodes as Spot Instances, expressed in USD.</p>"]
#[serde(rename="BidPrice")]
pub bid_price: Option<XmlStringMaxLen256>,
#[doc="<p>The date/time the instance group was created.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Date,
#[doc="<p>The date/time the instance group was terminated.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>Unique identifier for the instance group.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: Option<XmlStringMaxLen256>,
#[doc="<p>Target number of instances to run in the instance group.</p>"]
#[serde(rename="InstanceRequestCount")]
pub instance_request_count: Integer,
#[doc="<p>Instance group role in the cluster</p>"]
#[serde(rename="InstanceRole")]
pub instance_role: InstanceRoleType,
#[doc="<p>Actual count of running instances.</p>"]
#[serde(rename="InstanceRunningCount")]
pub instance_running_count: Integer,
#[doc="<p>EC2 instance type.</p>"]
#[serde(rename="InstanceType")]
pub instance_type: InstanceType,
#[doc="<p>Details regarding the state of the instance group.</p>"]
#[serde(rename="LastStateChangeReason")]
pub last_state_change_reason: Option<XmlString>,
#[doc="<p>Market type of the EC2 instances used to create a cluster node.</p>"]
#[serde(rename="Market")]
pub market: MarketType,
#[doc="<p>Friendly name for the instance group.</p>"]
#[serde(rename="Name")]
pub name: Option<XmlStringMaxLen256>,
#[doc="<p>The date/time the instance group was available to the cluster.</p>"]
#[serde(rename="ReadyDateTime")]
pub ready_date_time: Option<Date>,
#[doc="<p>The date/time the instance group was started.</p>"]
#[serde(rename="StartDateTime")]
pub start_date_time: Option<Date>,
#[doc="<p>State of instance group. The following values are deprecated: STARTING, TERMINATED, and FAILED.</p>"]
#[serde(rename="State")]
pub state: InstanceGroupState,
            }
            
pub type InstanceGroupDetailList = Vec<InstanceGroupDetail>;
pub type InstanceGroupId = String;
pub type InstanceGroupIdsList = Vec<XmlStringMaxLen256>;
pub type InstanceGroupList = Vec<InstanceGroup>;
#[doc="<p>Modify an instance group size.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InstanceGroupModifyConfig {
                #[doc="<p>The EC2 InstanceIds to terminate. After you terminate the instances, the instance group will not return to its original requested size.</p>"]
#[serde(rename="EC2InstanceIdsToTerminate")]
pub ec2_instance_ids_to_terminate: Option<EC2InstanceIdsToTerminateList>,
#[doc="<p>Target size for the instance group.</p>"]
#[serde(rename="InstanceCount")]
pub instance_count: Option<Integer>,
#[doc="<p>Unique ID of the instance group to expand or shrink.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: XmlStringMaxLen256,
#[doc="<p>Policy for customizing shrink operations.</p>"]
#[serde(rename="ShrinkPolicy")]
pub shrink_policy: Option<ShrinkPolicy>,
            }
            
pub type InstanceGroupModifyConfigList = Vec<InstanceGroupModifyConfig>;
pub type InstanceGroupState = String;
#[doc="<p>The status change reason details for the instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceGroupStateChangeReason {
                #[doc="<p>The programmable code for the state change reason.</p>"]
#[serde(rename="Code")]
pub code: Option<InstanceGroupStateChangeReasonCode>,
#[doc="<p>The status change reason description.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
            }
            
pub type InstanceGroupStateChangeReasonCode = String;
#[doc="<p>The details of the instance group status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceGroupStatus {
                #[doc="<p>The current state of the instance group.</p>"]
#[serde(rename="State")]
pub state: Option<InstanceGroupState>,
#[doc="<p>The status change reason details for the instance group.</p>"]
#[serde(rename="StateChangeReason")]
pub state_change_reason: Option<InstanceGroupStateChangeReason>,
#[doc="<p>The timeline of the instance group status over time.</p>"]
#[serde(rename="Timeline")]
pub timeline: Option<InstanceGroupTimeline>,
            }
            
#[doc="<p>The timeline of the instance group lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceGroupTimeline {
                #[doc="<p>The creation date and time of the instance group.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The date and time when the instance group terminated.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>The date and time when the instance group became ready to perform tasks.</p>"]
#[serde(rename="ReadyDateTime")]
pub ready_date_time: Option<Date>,
            }
            
pub type InstanceGroupType = String;
pub type InstanceGroupTypeList = Vec<InstanceGroupType>;
pub type InstanceId = String;
pub type InstanceList = Vec<Instance>;
#[doc="<p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct InstanceResizePolicy {
                #[doc="<p>Decommissioning timeout override for the specific list of instances to be terminated.</p>"]
#[serde(rename="InstanceTerminationTimeout")]
pub instance_termination_timeout: Option<Integer>,
#[doc="<p>Specific list of instances to be protected when shrinking an instance group.</p>"]
#[serde(rename="InstancesToProtect")]
pub instances_to_protect: Option<EC2InstanceIdsList>,
#[doc="<p>Specific list of instances to be terminated when shrinking an instance group.</p>"]
#[serde(rename="InstancesToTerminate")]
pub instances_to_terminate: Option<EC2InstanceIdsList>,
            }
            
pub type InstanceRoleType = String;
pub type InstanceState = String;
#[doc="<p>The details of the status change reason for the instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceStateChangeReason {
                #[doc="<p>The programmable code for the state change reason.</p>"]
#[serde(rename="Code")]
pub code: Option<InstanceStateChangeReasonCode>,
#[doc="<p>The status change reason description.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
            }
            
pub type InstanceStateChangeReasonCode = String;
pub type InstanceStateList = Vec<InstanceState>;
#[doc="<p>The instance status details.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceStatus {
                #[doc="<p>The current state of the instance.</p>"]
#[serde(rename="State")]
pub state: Option<InstanceState>,
#[doc="<p>The details of the status change reason for the instance.</p>"]
#[serde(rename="StateChangeReason")]
pub state_change_reason: Option<InstanceStateChangeReason>,
#[doc="<p>The timeline of the instance status over time.</p>"]
#[serde(rename="Timeline")]
pub timeline: Option<InstanceTimeline>,
            }
            
#[doc="<p>The timeline of the instance lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceTimeline {
                #[doc="<p>The creation date and time of the instance.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The date and time when the instance was terminated.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>The date and time when the instance was ready to perform tasks.</p>"]
#[serde(rename="ReadyDateTime")]
pub ready_date_time: Option<Date>,
            }
            
pub type InstanceType = String;
pub type Integer = i64;
#[doc="<p>A description of a job flow.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct JobFlowDetail {
                #[doc="<p>The version of the AMI used to initialize Amazon EC2 instances in the job flow. For a list of AMI versions currently supported by Amazon EMR, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/EnvironmentConfig_AMIVersion.html#ami-versions-supported\">AMI Versions Supported in EMR</a> in the <i>Amazon EMR Developer Guide.</i> </p>"]
#[serde(rename="AmiVersion")]
pub ami_version: Option<XmlStringMaxLen256>,
#[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides a way for the automatic scaling feature to get the required permissions it needs to launch and terminate EC2 instances in an instance group.</p>"]
#[serde(rename="AutoScalingRole")]
pub auto_scaling_role: Option<XmlString>,
#[doc="<p>A list of the bootstrap actions run by the job flow.</p>"]
#[serde(rename="BootstrapActions")]
pub bootstrap_actions: Option<BootstrapActionDetailList>,
#[doc="<p>Describes the execution status of the job flow.</p>"]
#[serde(rename="ExecutionStatusDetail")]
pub execution_status_detail: JobFlowExecutionStatusDetail,
#[doc="<p>Describes the Amazon EC2 instances of the job flow.</p>"]
#[serde(rename="Instances")]
pub instances: JobFlowInstancesDetail,
#[doc="<p>The job flow identifier.</p>"]
#[serde(rename="JobFlowId")]
pub job_flow_id: XmlStringMaxLen256,
#[doc="<p>The IAM role that was specified when the job flow was launched. The EC2 instances of the job flow assume this role.</p>"]
#[serde(rename="JobFlowRole")]
pub job_flow_role: Option<XmlString>,
#[doc="<p>The location in Amazon S3 where log files for the job are stored.</p>"]
#[serde(rename="LogUri")]
pub log_uri: Option<XmlString>,
#[doc="<p>The name of the job flow.</p>"]
#[serde(rename="Name")]
pub name: XmlStringMaxLen256,
#[doc="<p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
#[serde(rename="ScaleDownBehavior")]
pub scale_down_behavior: Option<ScaleDownBehavior>,
#[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<XmlString>,
#[doc="<p>A list of steps run by the job flow.</p>"]
#[serde(rename="Steps")]
pub steps: Option<StepDetailList>,
#[doc="<p>A list of strings set by third party software when the job flow is launched. If you are not using third party software to manage the job flow this value is empty.</p>"]
#[serde(rename="SupportedProducts")]
pub supported_products: Option<SupportedProductsList>,
#[doc="<p>Specifies whether the job flow is visible to all IAM users of the AWS account associated with the job flow. If this value is set to <code>true</code>, all IAM users of that AWS account can view and (if they have the proper policy permissions set) manage the job flow. If it is set to <code>false</code>, only the IAM user that created the job flow can view and manage it. This value can be changed using the <a>SetVisibleToAllUsers</a> action.</p>"]
#[serde(rename="VisibleToAllUsers")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub visible_to_all_users: Option<Boolean>,
            }
            
pub type JobFlowDetailList = Vec<JobFlowDetail>;
#[doc="<p>The type of instance.</p>"]
pub type JobFlowExecutionState = String;
pub type JobFlowExecutionStateList = Vec<JobFlowExecutionState>;
#[doc="<p>Describes the status of the job flow.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct JobFlowExecutionStatusDetail {
                #[doc="<p>The creation date and time of the job flow.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Date,
#[doc="<p>The completion date and time of the job flow.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>Description of the job flow last changed state.</p>"]
#[serde(rename="LastStateChangeReason")]
pub last_state_change_reason: Option<XmlString>,
#[doc="<p>The date and time when the job flow was ready to start running bootstrap actions.</p>"]
#[serde(rename="ReadyDateTime")]
pub ready_date_time: Option<Date>,
#[doc="<p>The start date and time of the job flow.</p>"]
#[serde(rename="StartDateTime")]
pub start_date_time: Option<Date>,
#[doc="<p>The state of the job flow.</p>"]
#[serde(rename="State")]
pub state: JobFlowExecutionState,
            }
            
#[doc="<p>A description of the Amazon EC2 instance running the job flow. A valid JobFlowInstancesConfig must contain at least InstanceGroups, which is the recommended configuration. However, a valid alternative is to have MasterInstanceType, SlaveInstanceType, and InstanceCount (all three must be present).</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct JobFlowInstancesConfig {
                #[doc="<p>A list of additional Amazon EC2 security group IDs for the master node.</p>"]
#[serde(rename="AdditionalMasterSecurityGroups")]
pub additional_master_security_groups: Option<SecurityGroupsList>,
#[doc="<p>A list of additional Amazon EC2 security group IDs for the slave nodes.</p>"]
#[serde(rename="AdditionalSlaveSecurityGroups")]
pub additional_slave_security_groups: Option<SecurityGroupsList>,
#[doc="<p>The name of the EC2 key pair that can be used to ssh to the master node as the user called \"hadoop.\"</p>"]
#[serde(rename="Ec2KeyName")]
pub ec_2_key_name: Option<XmlStringMaxLen256>,
#[doc="<p>To launch the job flow in Amazon Virtual Private Cloud (Amazon VPC), set this parameter to the identifier of the Amazon VPC subnet where you want the job flow to launch. If you do not specify this value, the job flow is launched in the normal Amazon Web Services cloud, outside of an Amazon VPC.</p> <p>Amazon VPC currently does not support cluster compute quadruple extra large (cc1.4xlarge) instances. Thus you cannot specify the cc1.4xlarge instance type for nodes of a job flow launched in a Amazon VPC.</p>"]
#[serde(rename="Ec2SubnetId")]
pub ec_2_subnet_id: Option<XmlStringMaxLen256>,
#[doc="<p>The identifier of the Amazon EC2 security group for the master node.</p>"]
#[serde(rename="EmrManagedMasterSecurityGroup")]
pub emr_managed_master_security_group: Option<XmlStringMaxLen256>,
#[doc="<p>The identifier of the Amazon EC2 security group for the slave nodes.</p>"]
#[serde(rename="EmrManagedSlaveSecurityGroup")]
pub emr_managed_slave_security_group: Option<XmlStringMaxLen256>,
#[doc="<p>The Hadoop version for the job flow. Valid inputs are \"0.18\" (deprecated), \"0.20\" (deprecated), \"0.20.205\" (deprecated), \"1.0.3\", \"2.2.0\", or \"2.4.0\". If you do not set this value, the default of 0.18 is used, unless the AmiVersion parameter is set in the RunJobFlow call, in which case the default version of Hadoop for that AMI version is used.</p>"]
#[serde(rename="HadoopVersion")]
pub hadoop_version: Option<XmlStringMaxLen256>,
#[doc="<p>The number of EC2 instances used to execute the job flow.</p>"]
#[serde(rename="InstanceCount")]
pub instance_count: Option<Integer>,
#[doc="<p>Configuration for the job flow's instance groups.</p>"]
#[serde(rename="InstanceGroups")]
pub instance_groups: Option<InstanceGroupConfigList>,
#[doc="<p>Specifies whether the job flow should be kept alive after completing all steps.</p>"]
#[serde(rename="KeepJobFlowAliveWhenNoSteps")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub keep_job_flow_alive_when_no_steps: Option<Boolean>,
#[doc="<p>The EC2 instance type of the master node.</p>"]
#[serde(rename="MasterInstanceType")]
pub master_instance_type: Option<InstanceType>,
#[doc="<p>The Availability Zone the job flow will run in.</p>"]
#[serde(rename="Placement")]
pub placement: Option<PlacementType>,
#[doc="<p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>"]
#[serde(rename="ServiceAccessSecurityGroup")]
pub service_access_security_group: Option<XmlStringMaxLen256>,
#[doc="<p>The EC2 instance type of the slave nodes.</p>"]
#[serde(rename="SlaveInstanceType")]
pub slave_instance_type: Option<InstanceType>,
#[doc="<p>Specifies whether to lock the job flow to prevent the Amazon EC2 instances from being terminated by API call, user intervention, or in the event of a job flow error.</p>"]
#[serde(rename="TerminationProtected")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub termination_protected: Option<Boolean>,
            }
            
#[doc="<p>Specify the type of Amazon EC2 instances to run the job flow on.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct JobFlowInstancesDetail {
                #[doc="<p>The name of an Amazon EC2 key pair that can be used to ssh to the master node of job flow.</p>"]
#[serde(rename="Ec2KeyName")]
pub ec_2_key_name: Option<XmlStringMaxLen256>,
#[doc="<p>For job flows launched within Amazon Virtual Private Cloud, this value specifies the identifier of the subnet where the job flow was launched.</p>"]
#[serde(rename="Ec2SubnetId")]
pub ec_2_subnet_id: Option<XmlStringMaxLen256>,
#[doc="<p>The Hadoop version for the job flow.</p>"]
#[serde(rename="HadoopVersion")]
pub hadoop_version: Option<XmlStringMaxLen256>,
#[doc="<p>The number of Amazon EC2 instances in the cluster. If the value is 1, the same instance serves as both the master and slave node. If the value is greater than 1, one instance is the master node and all others are slave nodes.</p>"]
#[serde(rename="InstanceCount")]
pub instance_count: Integer,
#[doc="<p>Details about the job flow's instance groups.</p>"]
#[serde(rename="InstanceGroups")]
pub instance_groups: Option<InstanceGroupDetailList>,
#[doc="<p>Specifies whether the job flow should terminate after completing all steps.</p>"]
#[serde(rename="KeepJobFlowAliveWhenNoSteps")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub keep_job_flow_alive_when_no_steps: Option<Boolean>,
#[doc="<p>The Amazon EC2 instance identifier of the master node.</p>"]
#[serde(rename="MasterInstanceId")]
pub master_instance_id: Option<XmlString>,
#[doc="<p>The Amazon EC2 master node instance type.</p>"]
#[serde(rename="MasterInstanceType")]
pub master_instance_type: InstanceType,
#[doc="<p>The DNS name of the master node.</p>"]
#[serde(rename="MasterPublicDnsName")]
pub master_public_dns_name: Option<XmlString>,
#[doc="<p>An approximation of the cost of the job flow, represented in m1.small/hours. This value is incremented one time for every hour that an m1.small runs. Larger instances are weighted more, so an Amazon EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
#[serde(rename="NormalizedInstanceHours")]
pub normalized_instance_hours: Option<Integer>,
#[doc="<p>The Amazon EC2 Availability Zone for the job flow.</p>"]
#[serde(rename="Placement")]
pub placement: Option<PlacementType>,
#[doc="<p>The Amazon EC2 slave node instance type.</p>"]
#[serde(rename="SlaveInstanceType")]
pub slave_instance_type: InstanceType,
#[doc="<p>Specifies whether the Amazon EC2 instances in the cluster are protected from termination by API calls, user intervention, or in the event of a job flow error.</p>"]
#[serde(rename="TerminationProtected")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub termination_protected: Option<Boolean>,
            }
            
#[doc="<p>A key value pair.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct KeyValue {
                #[doc="<p>The unique identifier of a key value pair.</p>"]
#[serde(rename="Key")]
pub key: Option<XmlString>,
#[doc="<p>The value part of the identified key.</p>"]
#[serde(rename="Value")]
pub value: Option<XmlString>,
            }
            
pub type KeyValueList = Vec<KeyValue>;
#[doc="<p>This input determines which bootstrap actions to retrieve.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListBootstrapActionsInput {
                #[doc="<p>The cluster identifier for the bootstrap actions to list.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This output contains the bootstrap actions detail.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListBootstrapActionsOutput {
                #[doc="<p>The bootstrap actions associated with the cluster.</p>"]
#[serde(rename="BootstrapActions")]
pub bootstrap_actions: Option<CommandList>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This input determines how the ListClusters action filters the list of clusters that it returns.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListClustersInput {
                #[doc="<p>The cluster state filters to apply when listing clusters.</p>"]
#[serde(rename="ClusterStates")]
pub cluster_states: Option<ClusterStateList>,
#[doc="<p>The creation date and time beginning value filter for listing clusters.</p>"]
#[serde(rename="CreatedAfter")]
pub created_after: Option<Date>,
#[doc="<p>The creation date and time end value filter for listing clusters.</p>"]
#[serde(rename="CreatedBefore")]
pub created_before: Option<Date>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This contains a ClusterSummaryList with the cluster details; for example, the cluster IDs, names, and status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListClustersOutput {
                #[doc="<p>The list of clusters for the account based on the given filters.</p>"]
#[serde(rename="Clusters")]
pub clusters: Option<ClusterSummaryList>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This input determines which instance groups to retrieve.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListInstanceGroupsInput {
                #[doc="<p>The identifier of the cluster for which to list the instance groups.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This input determines which instance groups to retrieve.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListInstanceGroupsOutput {
                #[doc="<p>The list of instance groups for the cluster and given filters.</p>"]
#[serde(rename="InstanceGroups")]
pub instance_groups: Option<InstanceGroupList>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This input determines which instances to list.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListInstancesInput {
                #[doc="<p>The identifier of the cluster for which to list the instances.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>The identifier of the instance group for which to list the instances.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: Option<InstanceGroupId>,
#[doc="<p>The type of instance group for which to list the instances.</p>"]
#[serde(rename="InstanceGroupTypes")]
pub instance_group_types: Option<InstanceGroupTypeList>,
#[doc="<p>A list of instance states that will filter the instances returned with this request.</p>"]
#[serde(rename="InstanceStates")]
pub instance_states: Option<InstanceStateList>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[doc="<p>This output contains the list of instances.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListInstancesOutput {
                #[doc="<p>The list of instances for the cluster and given filters.</p>"]
#[serde(rename="Instances")]
pub instances: Option<InstanceList>,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListSecurityConfigurationsInput {
                #[doc="<p>The pagination token that indicates the set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListSecurityConfigurationsOutput {
                #[doc="<p>A pagination token that indicates the next set of results to retrieve. Include the marker in the next ListSecurityConfiguration call to retrieve the next page of results, if required.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
#[doc="<p>The creation date and time, and name, of each security configuration.</p>"]
#[serde(rename="SecurityConfigurations")]
pub security_configurations: Option<SecurityConfigurationList>,
            }
            
#[doc="<p>This input determines which steps to list.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListStepsInput {
                #[doc="<p>The identifier of the cluster for which to list the steps.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
#[doc="<p>The filter to limit the step list based on the identifier of the steps.</p>"]
#[serde(rename="StepIds")]
pub step_ids: Option<XmlStringList>,
#[doc="<p>The filter to limit the step list based on certain states.</p>"]
#[serde(rename="StepStates")]
pub step_states: Option<StepStateList>,
            }
            
#[doc="<p>This output contains the list of steps returned in reverse order. This means that the last step is the first element in the list.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListStepsOutput {
                #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
#[serde(rename="Marker")]
pub marker: Option<Marker>,
#[doc="<p>The filtered list of steps for the cluster.</p>"]
#[serde(rename="Steps")]
pub steps: Option<StepSummaryList>,
            }
            
pub type Marker = String;
pub type MarketType = String;
#[doc="<p>A CloudWatch dimension, which is specified using a <code>Key</code> (known as a <code>Name</code> in CloudWatch), Value pair. By default, Amazon EMR uses one dimension whose <code>Key</code> is <code>JobFlowID</code> and <code>Value</code> is a variable representing the cluster ID, which is <code>${emr:cluster_id}</code>. This enables the rule to bootstrap when the cluster ID becomes available, and also enables a single automatic scaling policy to be reused for multiple clusters and instance groups.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct MetricDimension {
                #[doc="<p>The dimension name.</p>"]
#[serde(rename="Key")]
pub key: Option<String>,
#[doc="<p>The dimension value.</p>"]
#[serde(rename="Value")]
pub value: Option<String>,
            }
            
pub type MetricDimensionList = Vec<MetricDimension>;
#[doc="<p>Change the size of some instance groups.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ModifyInstanceGroupsInput {
                #[doc="<p>The ID of the cluster to which the instance group belongs.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: Option<ClusterId>,
#[doc="<p>Instance groups to change.</p>"]
#[serde(rename="InstanceGroups")]
pub instance_groups: Option<InstanceGroupModifyConfigList>,
            }
            
pub type NewSupportedProductsList = Vec<SupportedProductConfig>;
pub type NonNegativeDouble = f64;
#[doc="<p>The Amazon EC2 location for the job flow.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PlacementType {
                #[doc="<p>The Amazon EC2 Availability Zone for the job flow.</p>"]
#[serde(rename="AvailabilityZone")]
pub availability_zone: XmlString,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutAutoScalingPolicyInput {
                #[doc="<p>Specifies the definition of the automatic scaling policy.</p>"]
#[serde(rename="AutoScalingPolicy")]
pub auto_scaling_policy: AutoScalingPolicy,
#[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>Specifies the ID of the instance group to which the automatic scaling policy is applied.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: InstanceGroupId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutAutoScalingPolicyOutput {
                #[doc="<p>The automatic scaling policy definition.</p>"]
#[serde(rename="AutoScalingPolicy")]
pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
#[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: Option<ClusterId>,
#[doc="<p>Specifies the ID of the instance group to which the scaling policy is applied.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: Option<InstanceGroupId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveAutoScalingPolicyInput {
                #[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
#[serde(rename="ClusterId")]
pub cluster_id: ClusterId,
#[doc="<p>Specifies the ID of the instance group to which the scaling policy is applied.</p>"]
#[serde(rename="InstanceGroupId")]
pub instance_group_id: InstanceGroupId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveAutoScalingPolicyOutput;
            
#[doc="<p>This input identifies a cluster and a list of tags to remove.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTagsInput {
                #[doc="<p>The Amazon EMR resource identifier from which tags will be removed. This value must be a cluster identifier.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: ResourceId,
#[doc="<p>A list of tag keys to remove from a resource.</p>"]
#[serde(rename="TagKeys")]
pub tag_keys: StringList,
            }
            
#[doc="<p>This output indicates the result of removing tags from a resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveTagsOutput;
            
pub type ResourceId = String;
#[doc="<p> Input to the <a>RunJobFlow</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RunJobFlowInput {
                #[doc="<p>A JSON string for selecting additional features.</p>"]
#[serde(rename="AdditionalInfo")]
pub additional_info: Option<XmlString>,
#[doc="<note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and greater, use ReleaseLabel.</p> </note> <p>The version of the Amazon Machine Image (AMI) to use when launching Amazon EC2 instances in the job flow. The following values are valid:</p> <ul> <li> <p>The version number of the AMI to use, for example, \"2.0.\"</p> </li> </ul> <p>If the AMI supports multiple versions of Hadoop (for example, AMI 1.0 supports both Hadoop 0.18 and 0.20) you can use the <a>JobFlowInstancesConfig</a> <code>HadoopVersion</code> parameter to modify the version of Hadoop from the defaults shown above.</p> <p>For details about the AMI versions currently supported by Amazon Elastic MapReduce, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/EnvironmentConfig_AMIVersion.html#ami-versions-supported\">AMI Versions Supported in Elastic MapReduce</a> in the <i>Amazon Elastic MapReduce Developer Guide.</i> </p> <note> <p>Previously, the EMR AMI version API parameter options allowed you to use latest for the latest AMI version rather than specify a numerical value. Some regions no longer support this deprecated option as they only have a newer release label version of EMR, which requires you to specify an EMR release label release (EMR 4.x or later).</p> </note>"]
#[serde(rename="AmiVersion")]
pub ami_version: Option<XmlStringMaxLen256>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>A list of applications for the cluster. Valid values are: \"Hadoop\", \"Hive\", \"Mahout\", \"Pig\", and \"Spark.\" They are case insensitive.</p>"]
#[serde(rename="Applications")]
pub applications: Option<ApplicationList>,
#[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>"]
#[serde(rename="AutoScalingRole")]
pub auto_scaling_role: Option<XmlString>,
#[doc="<p>A list of bootstrap actions that will be run before Hadoop is started on the cluster nodes.</p>"]
#[serde(rename="BootstrapActions")]
pub bootstrap_actions: Option<BootstrapActionConfigList>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for the EMR cluster you are creating.</p>"]
#[serde(rename="Configurations")]
pub configurations: Option<ConfigurationList>,
#[doc="<p>A specification of the number and type of Amazon EC2 instances on which to run the job flow.</p>"]
#[serde(rename="Instances")]
pub instances: JobFlowInstancesConfig,
#[doc="<p>Also called instance profile and EC2 role. An IAM role for an EMR cluster. The EC2 instances of the cluster assume this role. The default role is <code>EMR_EC2_DefaultRole</code>. In order to use the default role, you must have already created it using the CLI or console.</p>"]
#[serde(rename="JobFlowRole")]
pub job_flow_role: Option<XmlString>,
#[doc="<p>The location in Amazon S3 to write the log files of the job flow. If a value is not provided, logs are not created.</p>"]
#[serde(rename="LogUri")]
pub log_uri: Option<XmlString>,
#[doc="<p>The name of the job flow.</p>"]
#[serde(rename="Name")]
pub name: XmlStringMaxLen256,
#[doc="<note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and greater, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use with the job flow that accepts a user argument list. EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action arguments. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-mapr.html\">Launch a Job Flow on the MapR Distribution for Hadoop</a>. Currently supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the cluster using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the cluster using MapR M5 Edition.</p> </li> <li> <p>\"mapr\" with the user arguments specifying \"--edition,m3\" or \"--edition,m5\" - launch the job flow using MapR M3 or M5 Edition respectively.</p> </li> <li> <p>\"mapr-m7\" - launch the cluster using MapR M7 Edition.</p> </li> <li> <p>\"hunk\" - launch the cluster with the Hunk Big Data Analtics Platform.</p> </li> <li> <p>\"hue\"- launch the cluster with Hue installed.</p> </li> <li> <p>\"spark\" - launch the cluster with Apache Spark installed.</p> </li> <li> <p>\"ganglia\" - launch the cluster with the Ganglia Monitoring System installed.</p> </li> </ul>"]
#[serde(rename="NewSupportedProducts")]
pub new_supported_products: Option<NewSupportedProductsList>,
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The release label for the Amazon EMR release. For Amazon EMR 3.x and 2.x AMIs, use amiVersion instead instead of ReleaseLabel.</p>"]
#[serde(rename="ReleaseLabel")]
pub release_label: Option<XmlStringMaxLen256>,
#[doc="<p>Specifies the way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
#[serde(rename="ScaleDownBehavior")]
pub scale_down_behavior: Option<ScaleDownBehavior>,
#[doc="<p>The name of a security configuration to apply to the cluster.</p>"]
#[serde(rename="SecurityConfiguration")]
pub security_configuration: Option<XmlString>,
#[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<XmlString>,
#[doc="<p>A list of steps to be executed by the job flow.</p>"]
#[serde(rename="Steps")]
pub steps: Option<StepConfigList>,
#[doc="<note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and greater, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use with the job flow. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-supported-products.html\">Use Third Party Applications with Amazon EMR</a>. Currently supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the job flow using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the job flow using MapR M5 Edition.</p> </li> </ul>"]
#[serde(rename="SupportedProducts")]
pub supported_products: Option<SupportedProductsList>,
#[doc="<p>A list of tags to associate with a cluster and propagate to Amazon EC2 instances.</p>"]
#[serde(rename="Tags")]
pub tags: Option<TagList>,
#[doc="<p>Whether the job flow is visible to all IAM users of the AWS account associated with the job flow. If this value is set to <code>true</code>, all IAM users of that AWS account can view and (if they have the proper policy permissions set) manage the job flow. If it is set to <code>false</code>, only the IAM user that created the job flow can view and manage it.</p>"]
#[serde(rename="VisibleToAllUsers")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub visible_to_all_users: Option<Boolean>,
            }
            
#[doc="<p> The result of the <a>RunJobFlow</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RunJobFlowOutput {
                #[doc="<p>An unique identifier for the job flow.</p>"]
#[serde(rename="JobFlowId")]
pub job_flow_id: Option<XmlStringMaxLen256>,
            }
            
pub type ScaleDownBehavior = String;
#[doc="<p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ScalingAction {
                #[doc="<p>Not available for instance groups. Instance groups use the market type specified for the group.</p>"]
#[serde(rename="Market")]
pub market: Option<MarketType>,
#[doc="<p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>"]
#[serde(rename="SimpleScalingPolicyConfiguration")]
pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
            }
            
#[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or below these limits.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ScalingConstraints {
                #[doc="<p>The upper boundary of EC2 instances in an instance group beyond which scaling activities are not allowed to grow. Scale-out activities will not add instances beyond this boundary.</p>"]
#[serde(rename="MaxCapacity")]
pub max_capacity: Integer,
#[doc="<p>The lower boundary of EC2 instances in an instance group below which scaling activities are not allowed to shrink. Scale-in activities will not terminate instances below this boundary.</p>"]
#[serde(rename="MinCapacity")]
pub min_capacity: Integer,
            }
            
#[doc="<p>A scale-in or scale-out rule that defines scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ScalingRule {
                #[doc="<p>The conditions that trigger an automatic scaling activity.</p>"]
#[serde(rename="Action")]
pub action: ScalingAction,
#[doc="<p>A friendly, more verbose description of the automatic scaling rule.</p>"]
#[serde(rename="Description")]
pub description: Option<String>,
#[doc="<p>The name used to identify an automatic scaling rule. Rule names must be unique within a scaling policy.</p>"]
#[serde(rename="Name")]
pub name: String,
#[doc="<p>The CloudWatch alarm definition that determines when automatic scaling activity is triggered.</p>"]
#[serde(rename="Trigger")]
pub trigger: ScalingTrigger,
            }
            
pub type ScalingRuleList = Vec<ScalingRule>;
#[doc="<p>The conditions that trigger an automatic scaling activity.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ScalingTrigger {
                #[doc="<p>The definition of a CloudWatch metric alarm. When the defined alarm conditions are met along with other trigger parameters, scaling activity begins.</p>"]
#[serde(rename="CloudWatchAlarmDefinition")]
pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
            }
            
#[doc="<p>Configuration of the script to run during a bootstrap action.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ScriptBootstrapActionConfig {
                #[doc="<p>A list of command line arguments to pass to the bootstrap action script.</p>"]
#[serde(rename="Args")]
pub args: Option<XmlStringList>,
#[doc="<p>Location of the script to run during a bootstrap action. Can be either a location in Amazon S3 or on a local file system.</p>"]
#[serde(rename="Path")]
pub path: XmlString,
            }
            
pub type SecurityConfigurationList = Vec<SecurityConfigurationSummary>;
#[doc="<p>The creation date and time, and name, of a security configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct SecurityConfigurationSummary {
                #[doc="<p>The date and time the security configuration was created.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The name of the security configuration.</p>"]
#[serde(rename="Name")]
pub name: Option<XmlString>,
            }
            
pub type SecurityGroupsList = Vec<XmlStringMaxLen256>;
#[doc="<p> The input argument to the <a>TerminationProtection</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct SetTerminationProtectionInput {
                #[doc="<p> A list of strings that uniquely identify the job flows to protect. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>DescribeJobFlows</a> . </p>"]
#[serde(rename="JobFlowIds")]
pub job_flow_ids: XmlStringList,
#[doc="<p>A Boolean that indicates whether to protect the job flow and prevent the Amazon EC2 instances in the cluster from shutting down due to API calls, user intervention, or job-flow error.</p>"]
#[serde(rename="TerminationProtected")]
pub termination_protected: Boolean,
            }
            
#[doc="<p>The input to the SetVisibleToAllUsers action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct SetVisibleToAllUsersInput {
                #[doc="<p>Identifiers of the job flows to receive the new visibility setting.</p>"]
#[serde(rename="JobFlowIds")]
pub job_flow_ids: XmlStringList,
#[doc="<p>Whether the specified job flows are visible to all IAM users of the AWS account associated with the job flow. If this value is set to True, all IAM users of that AWS account can view and, if they have the proper IAM policy permissions set, manage the job flows. If it is set to False, only the IAM user that created a job flow can view and manage it.</p>"]
#[serde(rename="VisibleToAllUsers")]
pub visible_to_all_users: Boolean,
            }
            
#[doc="<p>Policy for customizing shrink operations. Allows configuration of decommissioning timeout and targeted instance shrinking.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct ShrinkPolicy {
                #[doc="<p>The desired timeout for decommissioning an instance. Overrides the default YARN decommissioning timeout.</p>"]
#[serde(rename="DecommissionTimeout")]
pub decommission_timeout: Option<Integer>,
#[doc="<p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>"]
#[serde(rename="InstanceResizePolicy")]
pub instance_resize_policy: Option<InstanceResizePolicy>,
            }
            
#[doc="<p>An automatic scaling configuration, which describes how the policy adds or removes instances, the cooldown period, and the number of EC2 instances that will be added each time the CloudWatch metric alarm condition is satisfied.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct SimpleScalingPolicyConfiguration {
                #[doc="<p>The way in which EC2 instances are added (if <code>ScalingAdjustment</code> is a positive number) or terminated (if <code>ScalingAdjustment</code> is a negative number) each time the scaling activity is triggered. <code>CHANGE_IN_CAPACITY</code> is the default. <code>CHANGE_IN_CAPACITY</code> indicates that the EC2 instance count increments or decrements by <code>ScalingAdjustment</code>, which should be expressed as an integer. <code>PERCENT_CHANGE_IN_CAPACITY</code> indicates the instance count increments or decrements by the percentage specified by <code>ScalingAdjustment</code>, which should be expressed as a decimal, for example, 0.20 indicates an increase in 20% increments of cluster capacity. <code>EXACT_CAPACITY</code> indicates the scaling activity results in an instance group with the number of EC2 instances specified by <code>ScalingAdjustment</code>, which should be expressed as a positive integer.</p>"]
#[serde(rename="AdjustmentType")]
pub adjustment_type: Option<AdjustmentType>,
#[doc="<p>The amount of time, in seconds, after a scaling activity completes before any further trigger-related scaling activities can start. The default value is 0.</p>"]
#[serde(rename="CoolDown")]
pub cool_down: Option<Integer>,
#[doc="<p>The amount by which to scale in or scale out, based on the specified <code>AdjustmentType</code>. A positive value adds to the instance group's EC2 instance count while a negative number removes instances. If <code>AdjustmentType</code> is set to <code>EXACT_CAPACITY</code>, the number should only be a positive integer. If <code>AdjustmentType</code> is set to <code>PERCENT_CHANGE_IN_CAPACITY</code>, the value should express the percentage as a decimal. For example, -0.20 indicates a decrease in 20% increments of cluster capacity.</p>"]
#[serde(rename="ScalingAdjustment")]
pub scaling_adjustment: Integer,
            }
            
pub type Statistic = String;
#[doc="<p>This represents a step in a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Step {
                #[doc="<p>This specifies what action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE.</p>"]
#[serde(rename="ActionOnFailure")]
pub action_on_failure: Option<ActionOnFailure>,
#[doc="<p>The Hadoop job configuration of the cluster step.</p>"]
#[serde(rename="Config")]
pub config: Option<HadoopStepConfig>,
#[doc="<p>The identifier of the cluster step.</p>"]
#[serde(rename="Id")]
pub id: Option<StepId>,
#[doc="<p>The name of the cluster step.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>The current execution status details of the cluster step.</p>"]
#[serde(rename="Status")]
pub status: Option<StepStatus>,
            }
            
#[doc="<p>Specification of a job flow step.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct StepConfig {
                #[doc="<p>The action to take if the job flow step fails.</p>"]
#[serde(rename="ActionOnFailure")]
pub action_on_failure: Option<ActionOnFailure>,
#[doc="<p>The JAR file used for the job flow step.</p>"]
#[serde(rename="HadoopJarStep")]
pub hadoop_jar_step: HadoopJarStepConfig,
#[doc="<p>The name of the job flow step.</p>"]
#[serde(rename="Name")]
pub name: XmlStringMaxLen256,
            }
            
pub type StepConfigList = Vec<StepConfig>;
#[doc="<p>Combines the execution state and configuration of a step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepDetail {
                #[doc="<p>The description of the step status.</p>"]
#[serde(rename="ExecutionStatusDetail")]
pub execution_status_detail: StepExecutionStatusDetail,
#[doc="<p>The step configuration.</p>"]
#[serde(rename="StepConfig")]
pub step_config: StepConfig,
            }
            
pub type StepDetailList = Vec<StepDetail>;
pub type StepExecutionState = String;
#[doc="<p>The execution state of a step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepExecutionStatusDetail {
                #[doc="<p>The creation date and time of the step.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Date,
#[doc="<p>The completion date and time of the step.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>A description of the step's current state.</p>"]
#[serde(rename="LastStateChangeReason")]
pub last_state_change_reason: Option<XmlString>,
#[doc="<p>The start date and time of the step.</p>"]
#[serde(rename="StartDateTime")]
pub start_date_time: Option<Date>,
#[doc="<p>The state of the job flow step.</p>"]
#[serde(rename="State")]
pub state: StepExecutionState,
            }
            
pub type StepId = String;
pub type StepIdsList = Vec<XmlStringMaxLen256>;
pub type StepState = String;
#[doc="<p>The details of the step state change reason.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepStateChangeReason {
                #[doc="<p>The programmable code for the state change reason. Note: Currently, the service provides no code for the state change.</p>"]
#[serde(rename="Code")]
pub code: Option<StepStateChangeReasonCode>,
#[doc="<p>The descriptive message for the state change reason.</p>"]
#[serde(rename="Message")]
pub message: Option<String>,
            }
            
pub type StepStateChangeReasonCode = String;
pub type StepStateList = Vec<StepState>;
#[doc="<p>The execution status details of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepStatus {
                #[doc="<p>The details for the step failure including reason, message, and log file path where the root cause was identified.</p>"]
#[serde(rename="FailureDetails")]
pub failure_details: Option<FailureDetails>,
#[doc="<p>The execution state of the cluster step.</p>"]
#[serde(rename="State")]
pub state: Option<StepState>,
#[doc="<p>The reason for the step execution status change.</p>"]
#[serde(rename="StateChangeReason")]
pub state_change_reason: Option<StepStateChangeReason>,
#[doc="<p>The timeline of the cluster step status over time.</p>"]
#[serde(rename="Timeline")]
pub timeline: Option<StepTimeline>,
            }
            
#[doc="<p>The summary of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepSummary {
                #[doc="<p>This specifies what action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE.</p>"]
#[serde(rename="ActionOnFailure")]
pub action_on_failure: Option<ActionOnFailure>,
#[doc="<p>The Hadoop job configuration of the cluster step.</p>"]
#[serde(rename="Config")]
pub config: Option<HadoopStepConfig>,
#[doc="<p>The identifier of the cluster step.</p>"]
#[serde(rename="Id")]
pub id: Option<StepId>,
#[doc="<p>The name of the cluster step.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>The current execution status details of the cluster step.</p>"]
#[serde(rename="Status")]
pub status: Option<StepStatus>,
            }
            
pub type StepSummaryList = Vec<StepSummary>;
#[doc="<p>The timeline of the cluster step lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepTimeline {
                #[doc="<p>The date and time when the cluster step was created.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The date and time when the cluster step execution completed or failed.</p>"]
#[serde(rename="EndDateTime")]
pub end_date_time: Option<Date>,
#[doc="<p>The date and time when the cluster step execution started.</p>"]
#[serde(rename="StartDateTime")]
pub start_date_time: Option<Date>,
            }
            
pub type StringList = Vec<String>;
pub type StringMap = ::std::collections::HashMap<String, String>;
#[doc="<p>The list of supported product configurations which allow user-supplied arguments. EMR accepts these arguments and forwards them to the corresponding installation script as bootstrap action arguments.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct SupportedProductConfig {
                #[doc="<p>The list of user-supplied arguments.</p>"]
#[serde(rename="Args")]
pub args: Option<XmlStringList>,
#[doc="<p>The name of the product configuration.</p>"]
#[serde(rename="Name")]
pub name: Option<XmlStringMaxLen256>,
            }
            
pub type SupportedProductsList = Vec<XmlStringMaxLen256>;
#[doc="<p>A key/value pair containing user-defined metadata that you can associate with an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>A user-defined key, which is the minimum required information for a valid tag. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
#[serde(rename="Key")]
pub key: Option<String>,
#[doc="<p>A user-defined value, which is optional in a tag. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
#[serde(rename="Value")]
pub value: Option<String>,
            }
            
pub type TagList = Vec<Tag>;
#[doc="<p> Input to the <a>TerminateJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct TerminateJobFlowsInput {
                #[doc="<p>A list of job flows to be shutdown.</p>"]
#[serde(rename="JobFlowIds")]
pub job_flow_ids: XmlStringList,
            }
            
pub type Unit = String;
#[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct VolumeSpecification {
                #[doc="<p>The number of I/O operations per second (IOPS) that the volume supports.</p>"]
#[serde(rename="Iops")]
pub iops: Option<Integer>,
#[doc="<p>The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume type is EBS-optimized, the minimum value is 10.</p>"]
#[serde(rename="SizeInGB")]
pub size_in_gb: Integer,
#[doc="<p>The volume type. Volume types supported are gp2, io1, standard.</p>"]
#[serde(rename="VolumeType")]
pub volume_type: String,
            }
            
pub type XmlString = String;
pub type XmlStringList = Vec<XmlString>;
pub type XmlStringMaxLen256 = String;
/// Errors returned by AddInstanceGroups
                #[derive(Debug, PartialEq)]
                pub enum AddInstanceGroupsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddInstanceGroupsError {
                    pub fn from_body(body: &str) -> AddInstanceGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => AddInstanceGroupsError::InternalServerError(String::from(error_message)),
"ValidationException" => AddInstanceGroupsError::Validation(error_message.to_string()),
_ => AddInstanceGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddInstanceGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddInstanceGroupsError {
                    fn from(err: serde_json::error::Error) -> AddInstanceGroupsError {
                        AddInstanceGroupsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddInstanceGroupsError {
                    fn from(err: CredentialsError) -> AddInstanceGroupsError {
                        AddInstanceGroupsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddInstanceGroupsError {
                    fn from(err: HttpDispatchError) -> AddInstanceGroupsError {
                        AddInstanceGroupsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddInstanceGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddInstanceGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            AddInstanceGroupsError::InternalServerError(ref cause) => cause,
AddInstanceGroupsError::Validation(ref cause) => cause,
AddInstanceGroupsError::Credentials(ref err) => err.description(),
AddInstanceGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AddInstanceGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by AddJobFlowSteps
                #[derive(Debug, PartialEq)]
                pub enum AddJobFlowStepsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddJobFlowStepsError {
                    pub fn from_body(body: &str) -> AddJobFlowStepsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => AddJobFlowStepsError::InternalServerError(String::from(error_message)),
"ValidationException" => AddJobFlowStepsError::Validation(error_message.to_string()),
_ => AddJobFlowStepsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddJobFlowStepsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddJobFlowStepsError {
                    fn from(err: serde_json::error::Error) -> AddJobFlowStepsError {
                        AddJobFlowStepsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddJobFlowStepsError {
                    fn from(err: CredentialsError) -> AddJobFlowStepsError {
                        AddJobFlowStepsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddJobFlowStepsError {
                    fn from(err: HttpDispatchError) -> AddJobFlowStepsError {
                        AddJobFlowStepsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddJobFlowStepsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddJobFlowStepsError {
                    fn description(&self) -> &str {
                        match *self {
                            AddJobFlowStepsError::InternalServerError(ref cause) => cause,
AddJobFlowStepsError::Validation(ref cause) => cause,
AddJobFlowStepsError::Credentials(ref err) => err.description(),
AddJobFlowStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AddJobFlowStepsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by AddTags
                #[derive(Debug, PartialEq)]
                pub enum AddTagsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddTagsError {
                    pub fn from_body(body: &str) -> AddTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => AddTagsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => AddTagsError::InvalidRequest(String::from(error_message)),
"ValidationException" => AddTagsError::Validation(error_message.to_string()),
_ => AddTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddTagsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddTagsError {
                    fn from(err: serde_json::error::Error) -> AddTagsError {
                        AddTagsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddTagsError {
                    fn from(err: CredentialsError) -> AddTagsError {
                        AddTagsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddTagsError {
                    fn from(err: HttpDispatchError) -> AddTagsError {
                        AddTagsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddTagsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddTagsError {
                    fn description(&self) -> &str {
                        match *self {
                            AddTagsError::InternalServer(ref cause) => cause,
AddTagsError::InvalidRequest(ref cause) => cause,
AddTagsError::Validation(ref cause) => cause,
AddTagsError::Credentials(ref err) => err.description(),
AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
AddTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CancelSteps
                #[derive(Debug, PartialEq)]
                pub enum CancelStepsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CancelStepsError {
                    pub fn from_body(body: &str) -> CancelStepsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => CancelStepsError::InternalServerError(String::from(error_message)),
"InvalidRequestException" => CancelStepsError::InvalidRequest(String::from(error_message)),
"ValidationException" => CancelStepsError::Validation(error_message.to_string()),
_ => CancelStepsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CancelStepsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CancelStepsError {
                    fn from(err: serde_json::error::Error) -> CancelStepsError {
                        CancelStepsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CancelStepsError {
                    fn from(err: CredentialsError) -> CancelStepsError {
                        CancelStepsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CancelStepsError {
                    fn from(err: HttpDispatchError) -> CancelStepsError {
                        CancelStepsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CancelStepsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CancelStepsError {
                    fn description(&self) -> &str {
                        match *self {
                            CancelStepsError::InternalServerError(ref cause) => cause,
CancelStepsError::InvalidRequest(ref cause) => cause,
CancelStepsError::Validation(ref cause) => cause,
CancelStepsError::Credentials(ref err) => err.description(),
CancelStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CancelStepsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateSecurityConfiguration
                #[derive(Debug, PartialEq)]
                pub enum CreateSecurityConfigurationError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateSecurityConfigurationError {
                    pub fn from_body(body: &str) -> CreateSecurityConfigurationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => CreateSecurityConfigurationError::InternalServer(String::from(error_message)),
"InvalidRequestException" => CreateSecurityConfigurationError::InvalidRequest(String::from(error_message)),
"ValidationException" => CreateSecurityConfigurationError::Validation(error_message.to_string()),
_ => CreateSecurityConfigurationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateSecurityConfigurationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateSecurityConfigurationError {
                    fn from(err: serde_json::error::Error) -> CreateSecurityConfigurationError {
                        CreateSecurityConfigurationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateSecurityConfigurationError {
                    fn from(err: CredentialsError) -> CreateSecurityConfigurationError {
                        CreateSecurityConfigurationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateSecurityConfigurationError {
                    fn from(err: HttpDispatchError) -> CreateSecurityConfigurationError {
                        CreateSecurityConfigurationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateSecurityConfigurationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateSecurityConfigurationError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateSecurityConfigurationError::InternalServer(ref cause) => cause,
CreateSecurityConfigurationError::InvalidRequest(ref cause) => cause,
CreateSecurityConfigurationError::Validation(ref cause) => cause,
CreateSecurityConfigurationError::Credentials(ref err) => err.description(),
CreateSecurityConfigurationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
CreateSecurityConfigurationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteSecurityConfiguration
                #[derive(Debug, PartialEq)]
                pub enum DeleteSecurityConfigurationError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteSecurityConfigurationError {
                    pub fn from_body(body: &str) -> DeleteSecurityConfigurationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => DeleteSecurityConfigurationError::InternalServer(String::from(error_message)),
"InvalidRequestException" => DeleteSecurityConfigurationError::InvalidRequest(String::from(error_message)),
"ValidationException" => DeleteSecurityConfigurationError::Validation(error_message.to_string()),
_ => DeleteSecurityConfigurationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteSecurityConfigurationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteSecurityConfigurationError {
                    fn from(err: serde_json::error::Error) -> DeleteSecurityConfigurationError {
                        DeleteSecurityConfigurationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteSecurityConfigurationError {
                    fn from(err: CredentialsError) -> DeleteSecurityConfigurationError {
                        DeleteSecurityConfigurationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteSecurityConfigurationError {
                    fn from(err: HttpDispatchError) -> DeleteSecurityConfigurationError {
                        DeleteSecurityConfigurationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteSecurityConfigurationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteSecurityConfigurationError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteSecurityConfigurationError::InternalServer(ref cause) => cause,
DeleteSecurityConfigurationError::InvalidRequest(ref cause) => cause,
DeleteSecurityConfigurationError::Validation(ref cause) => cause,
DeleteSecurityConfigurationError::Credentials(ref err) => err.description(),
DeleteSecurityConfigurationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DeleteSecurityConfigurationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeCluster
                #[derive(Debug, PartialEq)]
                pub enum DescribeClusterError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeClusterError {
                    pub fn from_body(body: &str) -> DescribeClusterError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => DescribeClusterError::InternalServer(String::from(error_message)),
"InvalidRequestException" => DescribeClusterError::InvalidRequest(String::from(error_message)),
"ValidationException" => DescribeClusterError::Validation(error_message.to_string()),
_ => DescribeClusterError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeClusterError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeClusterError {
                    fn from(err: serde_json::error::Error) -> DescribeClusterError {
                        DescribeClusterError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeClusterError {
                    fn from(err: CredentialsError) -> DescribeClusterError {
                        DescribeClusterError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeClusterError {
                    fn from(err: HttpDispatchError) -> DescribeClusterError {
                        DescribeClusterError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeClusterError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeClusterError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeClusterError::InternalServer(ref cause) => cause,
DescribeClusterError::InvalidRequest(ref cause) => cause,
DescribeClusterError::Validation(ref cause) => cause,
DescribeClusterError::Credentials(ref err) => err.description(),
DescribeClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeClusterError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeJobFlows
                #[derive(Debug, PartialEq)]
                pub enum DescribeJobFlowsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeJobFlowsError {
                    pub fn from_body(body: &str) -> DescribeJobFlowsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeJobFlowsError::InternalServerError(String::from(error_message)),
"ValidationException" => DescribeJobFlowsError::Validation(error_message.to_string()),
_ => DescribeJobFlowsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeJobFlowsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeJobFlowsError {
                    fn from(err: serde_json::error::Error) -> DescribeJobFlowsError {
                        DescribeJobFlowsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeJobFlowsError {
                    fn from(err: CredentialsError) -> DescribeJobFlowsError {
                        DescribeJobFlowsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeJobFlowsError {
                    fn from(err: HttpDispatchError) -> DescribeJobFlowsError {
                        DescribeJobFlowsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeJobFlowsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeJobFlowsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeJobFlowsError::InternalServerError(ref cause) => cause,
DescribeJobFlowsError::Validation(ref cause) => cause,
DescribeJobFlowsError::Credentials(ref err) => err.description(),
DescribeJobFlowsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeJobFlowsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeSecurityConfiguration
                #[derive(Debug, PartialEq)]
                pub enum DescribeSecurityConfigurationError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeSecurityConfigurationError {
                    pub fn from_body(body: &str) -> DescribeSecurityConfigurationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => DescribeSecurityConfigurationError::InternalServer(String::from(error_message)),
"InvalidRequestException" => DescribeSecurityConfigurationError::InvalidRequest(String::from(error_message)),
"ValidationException" => DescribeSecurityConfigurationError::Validation(error_message.to_string()),
_ => DescribeSecurityConfigurationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeSecurityConfigurationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeSecurityConfigurationError {
                    fn from(err: serde_json::error::Error) -> DescribeSecurityConfigurationError {
                        DescribeSecurityConfigurationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeSecurityConfigurationError {
                    fn from(err: CredentialsError) -> DescribeSecurityConfigurationError {
                        DescribeSecurityConfigurationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeSecurityConfigurationError {
                    fn from(err: HttpDispatchError) -> DescribeSecurityConfigurationError {
                        DescribeSecurityConfigurationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeSecurityConfigurationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeSecurityConfigurationError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeSecurityConfigurationError::InternalServer(ref cause) => cause,
DescribeSecurityConfigurationError::InvalidRequest(ref cause) => cause,
DescribeSecurityConfigurationError::Validation(ref cause) => cause,
DescribeSecurityConfigurationError::Credentials(ref err) => err.description(),
DescribeSecurityConfigurationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeSecurityConfigurationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeStep
                #[derive(Debug, PartialEq)]
                pub enum DescribeStepError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeStepError {
                    pub fn from_body(body: &str) -> DescribeStepError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => DescribeStepError::InternalServer(String::from(error_message)),
"InvalidRequestException" => DescribeStepError::InvalidRequest(String::from(error_message)),
"ValidationException" => DescribeStepError::Validation(error_message.to_string()),
_ => DescribeStepError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeStepError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeStepError {
                    fn from(err: serde_json::error::Error) -> DescribeStepError {
                        DescribeStepError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeStepError {
                    fn from(err: CredentialsError) -> DescribeStepError {
                        DescribeStepError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeStepError {
                    fn from(err: HttpDispatchError) -> DescribeStepError {
                        DescribeStepError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeStepError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeStepError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeStepError::InternalServer(ref cause) => cause,
DescribeStepError::InvalidRequest(ref cause) => cause,
DescribeStepError::Validation(ref cause) => cause,
DescribeStepError::Credentials(ref err) => err.description(),
DescribeStepError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
DescribeStepError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListBootstrapActions
                #[derive(Debug, PartialEq)]
                pub enum ListBootstrapActionsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListBootstrapActionsError {
                    pub fn from_body(body: &str) -> ListBootstrapActionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListBootstrapActionsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListBootstrapActionsError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListBootstrapActionsError::Validation(error_message.to_string()),
_ => ListBootstrapActionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListBootstrapActionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListBootstrapActionsError {
                    fn from(err: serde_json::error::Error) -> ListBootstrapActionsError {
                        ListBootstrapActionsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListBootstrapActionsError {
                    fn from(err: CredentialsError) -> ListBootstrapActionsError {
                        ListBootstrapActionsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListBootstrapActionsError {
                    fn from(err: HttpDispatchError) -> ListBootstrapActionsError {
                        ListBootstrapActionsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListBootstrapActionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListBootstrapActionsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListBootstrapActionsError::InternalServer(ref cause) => cause,
ListBootstrapActionsError::InvalidRequest(ref cause) => cause,
ListBootstrapActionsError::Validation(ref cause) => cause,
ListBootstrapActionsError::Credentials(ref err) => err.description(),
ListBootstrapActionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListBootstrapActionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListClusters
                #[derive(Debug, PartialEq)]
                pub enum ListClustersError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListClustersError {
                    pub fn from_body(body: &str) -> ListClustersError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListClustersError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListClustersError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListClustersError::Validation(error_message.to_string()),
_ => ListClustersError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListClustersError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListClustersError {
                    fn from(err: serde_json::error::Error) -> ListClustersError {
                        ListClustersError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListClustersError {
                    fn from(err: CredentialsError) -> ListClustersError {
                        ListClustersError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListClustersError {
                    fn from(err: HttpDispatchError) -> ListClustersError {
                        ListClustersError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListClustersError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListClustersError {
                    fn description(&self) -> &str {
                        match *self {
                            ListClustersError::InternalServer(ref cause) => cause,
ListClustersError::InvalidRequest(ref cause) => cause,
ListClustersError::Validation(ref cause) => cause,
ListClustersError::Credentials(ref err) => err.description(),
ListClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListClustersError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListInstanceGroups
                #[derive(Debug, PartialEq)]
                pub enum ListInstanceGroupsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListInstanceGroupsError {
                    pub fn from_body(body: &str) -> ListInstanceGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListInstanceGroupsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListInstanceGroupsError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListInstanceGroupsError::Validation(error_message.to_string()),
_ => ListInstanceGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListInstanceGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListInstanceGroupsError {
                    fn from(err: serde_json::error::Error) -> ListInstanceGroupsError {
                        ListInstanceGroupsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListInstanceGroupsError {
                    fn from(err: CredentialsError) -> ListInstanceGroupsError {
                        ListInstanceGroupsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListInstanceGroupsError {
                    fn from(err: HttpDispatchError) -> ListInstanceGroupsError {
                        ListInstanceGroupsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListInstanceGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListInstanceGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListInstanceGroupsError::InternalServer(ref cause) => cause,
ListInstanceGroupsError::InvalidRequest(ref cause) => cause,
ListInstanceGroupsError::Validation(ref cause) => cause,
ListInstanceGroupsError::Credentials(ref err) => err.description(),
ListInstanceGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListInstanceGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListInstances
                #[derive(Debug, PartialEq)]
                pub enum ListInstancesError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListInstancesError {
                    pub fn from_body(body: &str) -> ListInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListInstancesError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListInstancesError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListInstancesError::Validation(error_message.to_string()),
_ => ListInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListInstancesError {
                    fn from(err: serde_json::error::Error) -> ListInstancesError {
                        ListInstancesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListInstancesError {
                    fn from(err: CredentialsError) -> ListInstancesError {
                        ListInstancesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListInstancesError {
                    fn from(err: HttpDispatchError) -> ListInstancesError {
                        ListInstancesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListInstancesError::InternalServer(ref cause) => cause,
ListInstancesError::InvalidRequest(ref cause) => cause,
ListInstancesError::Validation(ref cause) => cause,
ListInstancesError::Credentials(ref err) => err.description(),
ListInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListSecurityConfigurations
                #[derive(Debug, PartialEq)]
                pub enum ListSecurityConfigurationsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListSecurityConfigurationsError {
                    pub fn from_body(body: &str) -> ListSecurityConfigurationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListSecurityConfigurationsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListSecurityConfigurationsError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListSecurityConfigurationsError::Validation(error_message.to_string()),
_ => ListSecurityConfigurationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListSecurityConfigurationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListSecurityConfigurationsError {
                    fn from(err: serde_json::error::Error) -> ListSecurityConfigurationsError {
                        ListSecurityConfigurationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListSecurityConfigurationsError {
                    fn from(err: CredentialsError) -> ListSecurityConfigurationsError {
                        ListSecurityConfigurationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListSecurityConfigurationsError {
                    fn from(err: HttpDispatchError) -> ListSecurityConfigurationsError {
                        ListSecurityConfigurationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListSecurityConfigurationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListSecurityConfigurationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListSecurityConfigurationsError::InternalServer(ref cause) => cause,
ListSecurityConfigurationsError::InvalidRequest(ref cause) => cause,
ListSecurityConfigurationsError::Validation(ref cause) => cause,
ListSecurityConfigurationsError::Credentials(ref err) => err.description(),
ListSecurityConfigurationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListSecurityConfigurationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListSteps
                #[derive(Debug, PartialEq)]
                pub enum ListStepsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListStepsError {
                    pub fn from_body(body: &str) -> ListStepsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => ListStepsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => ListStepsError::InvalidRequest(String::from(error_message)),
"ValidationException" => ListStepsError::Validation(error_message.to_string()),
_ => ListStepsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListStepsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListStepsError {
                    fn from(err: serde_json::error::Error) -> ListStepsError {
                        ListStepsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListStepsError {
                    fn from(err: CredentialsError) -> ListStepsError {
                        ListStepsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListStepsError {
                    fn from(err: HttpDispatchError) -> ListStepsError {
                        ListStepsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListStepsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListStepsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListStepsError::InternalServer(ref cause) => cause,
ListStepsError::InvalidRequest(ref cause) => cause,
ListStepsError::Validation(ref cause) => cause,
ListStepsError::Credentials(ref err) => err.description(),
ListStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ListStepsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ModifyInstanceGroups
                #[derive(Debug, PartialEq)]
                pub enum ModifyInstanceGroupsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ModifyInstanceGroupsError {
                    pub fn from_body(body: &str) -> ModifyInstanceGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ModifyInstanceGroupsError::InternalServerError(String::from(error_message)),
"ValidationException" => ModifyInstanceGroupsError::Validation(error_message.to_string()),
_ => ModifyInstanceGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ModifyInstanceGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ModifyInstanceGroupsError {
                    fn from(err: serde_json::error::Error) -> ModifyInstanceGroupsError {
                        ModifyInstanceGroupsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ModifyInstanceGroupsError {
                    fn from(err: CredentialsError) -> ModifyInstanceGroupsError {
                        ModifyInstanceGroupsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ModifyInstanceGroupsError {
                    fn from(err: HttpDispatchError) -> ModifyInstanceGroupsError {
                        ModifyInstanceGroupsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ModifyInstanceGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ModifyInstanceGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            ModifyInstanceGroupsError::InternalServerError(ref cause) => cause,
ModifyInstanceGroupsError::Validation(ref cause) => cause,
ModifyInstanceGroupsError::Credentials(ref err) => err.description(),
ModifyInstanceGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
ModifyInstanceGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutAutoScalingPolicy
                #[derive(Debug, PartialEq)]
                pub enum PutAutoScalingPolicyError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutAutoScalingPolicyError {
                    pub fn from_body(body: &str) -> PutAutoScalingPolicyError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ValidationException" => PutAutoScalingPolicyError::Validation(error_message.to_string()),
_ => PutAutoScalingPolicyError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutAutoScalingPolicyError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutAutoScalingPolicyError {
                    fn from(err: serde_json::error::Error) -> PutAutoScalingPolicyError {
                        PutAutoScalingPolicyError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutAutoScalingPolicyError {
                    fn from(err: CredentialsError) -> PutAutoScalingPolicyError {
                        PutAutoScalingPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutAutoScalingPolicyError {
                    fn from(err: HttpDispatchError) -> PutAutoScalingPolicyError {
                        PutAutoScalingPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutAutoScalingPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutAutoScalingPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            PutAutoScalingPolicyError::Validation(ref cause) => cause,
PutAutoScalingPolicyError::Credentials(ref err) => err.description(),
PutAutoScalingPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
PutAutoScalingPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveAutoScalingPolicy
                #[derive(Debug, PartialEq)]
                pub enum RemoveAutoScalingPolicyError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveAutoScalingPolicyError {
                    pub fn from_body(body: &str) -> RemoveAutoScalingPolicyError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ValidationException" => RemoveAutoScalingPolicyError::Validation(error_message.to_string()),
_ => RemoveAutoScalingPolicyError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveAutoScalingPolicyError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveAutoScalingPolicyError {
                    fn from(err: serde_json::error::Error) -> RemoveAutoScalingPolicyError {
                        RemoveAutoScalingPolicyError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveAutoScalingPolicyError {
                    fn from(err: CredentialsError) -> RemoveAutoScalingPolicyError {
                        RemoveAutoScalingPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveAutoScalingPolicyError {
                    fn from(err: HttpDispatchError) -> RemoveAutoScalingPolicyError {
                        RemoveAutoScalingPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveAutoScalingPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveAutoScalingPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveAutoScalingPolicyError::Validation(ref cause) => cause,
RemoveAutoScalingPolicyError::Credentials(ref err) => err.description(),
RemoveAutoScalingPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
RemoveAutoScalingPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTags
                #[derive(Debug, PartialEq)]
                pub enum RemoveTagsError {
                    
///<p>This exception occurs when there is an internal failure in the EMR service.</p>
InternalServer(String),
///<p>This exception occurs when there is something wrong with user input.</p>
InvalidRequest(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTagsError {
                    pub fn from_body(body: &str) -> RemoveTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerException" => RemoveTagsError::InternalServer(String::from(error_message)),
"InvalidRequestException" => RemoveTagsError::InvalidRequest(String::from(error_message)),
"ValidationException" => RemoveTagsError::Validation(error_message.to_string()),
_ => RemoveTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTagsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTagsError {
                    fn from(err: serde_json::error::Error) -> RemoveTagsError {
                        RemoveTagsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveTagsError {
                    fn from(err: CredentialsError) -> RemoveTagsError {
                        RemoveTagsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveTagsError {
                    fn from(err: HttpDispatchError) -> RemoveTagsError {
                        RemoveTagsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveTagsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTagsError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTagsError::InternalServer(ref cause) => cause,
RemoveTagsError::InvalidRequest(ref cause) => cause,
RemoveTagsError::Validation(ref cause) => cause,
RemoveTagsError::Credentials(ref err) => err.description(),
RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
RemoveTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RunJobFlow
                #[derive(Debug, PartialEq)]
                pub enum RunJobFlowError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RunJobFlowError {
                    pub fn from_body(body: &str) -> RunJobFlowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => RunJobFlowError::InternalServerError(String::from(error_message)),
"ValidationException" => RunJobFlowError::Validation(error_message.to_string()),
_ => RunJobFlowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RunJobFlowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RunJobFlowError {
                    fn from(err: serde_json::error::Error) -> RunJobFlowError {
                        RunJobFlowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RunJobFlowError {
                    fn from(err: CredentialsError) -> RunJobFlowError {
                        RunJobFlowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RunJobFlowError {
                    fn from(err: HttpDispatchError) -> RunJobFlowError {
                        RunJobFlowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RunJobFlowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RunJobFlowError {
                    fn description(&self) -> &str {
                        match *self {
                            RunJobFlowError::InternalServerError(ref cause) => cause,
RunJobFlowError::Validation(ref cause) => cause,
RunJobFlowError::Credentials(ref err) => err.description(),
RunJobFlowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
RunJobFlowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by SetTerminationProtection
                #[derive(Debug, PartialEq)]
                pub enum SetTerminationProtectionError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl SetTerminationProtectionError {
                    pub fn from_body(body: &str) -> SetTerminationProtectionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => SetTerminationProtectionError::InternalServerError(String::from(error_message)),
"ValidationException" => SetTerminationProtectionError::Validation(error_message.to_string()),
_ => SetTerminationProtectionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => SetTerminationProtectionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for SetTerminationProtectionError {
                    fn from(err: serde_json::error::Error) -> SetTerminationProtectionError {
                        SetTerminationProtectionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for SetTerminationProtectionError {
                    fn from(err: CredentialsError) -> SetTerminationProtectionError {
                        SetTerminationProtectionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for SetTerminationProtectionError {
                    fn from(err: HttpDispatchError) -> SetTerminationProtectionError {
                        SetTerminationProtectionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for SetTerminationProtectionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for SetTerminationProtectionError {
                    fn description(&self) -> &str {
                        match *self {
                            SetTerminationProtectionError::InternalServerError(ref cause) => cause,
SetTerminationProtectionError::Validation(ref cause) => cause,
SetTerminationProtectionError::Credentials(ref err) => err.description(),
SetTerminationProtectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
SetTerminationProtectionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by SetVisibleToAllUsers
                #[derive(Debug, PartialEq)]
                pub enum SetVisibleToAllUsersError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl SetVisibleToAllUsersError {
                    pub fn from_body(body: &str) -> SetVisibleToAllUsersError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => SetVisibleToAllUsersError::InternalServerError(String::from(error_message)),
"ValidationException" => SetVisibleToAllUsersError::Validation(error_message.to_string()),
_ => SetVisibleToAllUsersError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => SetVisibleToAllUsersError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for SetVisibleToAllUsersError {
                    fn from(err: serde_json::error::Error) -> SetVisibleToAllUsersError {
                        SetVisibleToAllUsersError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for SetVisibleToAllUsersError {
                    fn from(err: CredentialsError) -> SetVisibleToAllUsersError {
                        SetVisibleToAllUsersError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for SetVisibleToAllUsersError {
                    fn from(err: HttpDispatchError) -> SetVisibleToAllUsersError {
                        SetVisibleToAllUsersError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for SetVisibleToAllUsersError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for SetVisibleToAllUsersError {
                    fn description(&self) -> &str {
                        match *self {
                            SetVisibleToAllUsersError::InternalServerError(ref cause) => cause,
SetVisibleToAllUsersError::Validation(ref cause) => cause,
SetVisibleToAllUsersError::Credentials(ref err) => err.description(),
SetVisibleToAllUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
SetVisibleToAllUsersError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by TerminateJobFlows
                #[derive(Debug, PartialEq)]
                pub enum TerminateJobFlowsError {
                    
///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl TerminateJobFlowsError {
                    pub fn from_body(body: &str) -> TerminateJobFlowsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => TerminateJobFlowsError::InternalServerError(String::from(error_message)),
"ValidationException" => TerminateJobFlowsError::Validation(error_message.to_string()),
_ => TerminateJobFlowsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => TerminateJobFlowsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for TerminateJobFlowsError {
                    fn from(err: serde_json::error::Error) -> TerminateJobFlowsError {
                        TerminateJobFlowsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for TerminateJobFlowsError {
                    fn from(err: CredentialsError) -> TerminateJobFlowsError {
                        TerminateJobFlowsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for TerminateJobFlowsError {
                    fn from(err: HttpDispatchError) -> TerminateJobFlowsError {
                        TerminateJobFlowsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for TerminateJobFlowsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for TerminateJobFlowsError {
                    fn description(&self) -> &str {
                        match *self {
                            TerminateJobFlowsError::InternalServerError(ref cause) => cause,
TerminateJobFlowsError::Validation(ref cause) => cause,
TerminateJobFlowsError::Credentials(ref err) => err.description(),
TerminateJobFlowsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
TerminateJobFlowsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the Amazon EMR API. Amazon EMR clients implement this trait.
        pub trait Emr {
        

                #[doc="<p>Adds one or more instance groups to a running cluster.</p>"]
                fn add_instance_groups(&self, input: &AddInstanceGroupsInput)  -> Result<AddInstanceGroupsOutput, AddInstanceGroupsError>;
                

                #[doc="<p>AddJobFlowSteps adds new steps to a running job flow. A maximum of 256 steps are allowed in each job flow.</p> <p>If your job flow is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Job Flow</a> in the <i>Amazon EMR Developer's Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the job flow or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a job flow that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>"]
                fn add_job_flow_steps(&self, input: &AddJobFlowStepsInput)  -> Result<AddJobFlowStepsOutput, AddJobFlowStepsError>;
                

                #[doc="<p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
                fn add_tags(&self, input: &AddTagsInput)  -> Result<AddTagsOutput, AddTagsError>;
                

                #[doc="<p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>"]
                fn cancel_steps(&self, input: &CancelStepsInput)  -> Result<CancelStepsOutput, CancelStepsError>;
                

                #[doc="<p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>"]
                fn create_security_configuration(&self, input: &CreateSecurityConfigurationInput)  -> Result<CreateSecurityConfigurationOutput, CreateSecurityConfigurationError>;
                

                #[doc="<p>Deletes a security configuration.</p>"]
                fn delete_security_configuration(&self, input: &DeleteSecurityConfigurationInput)  -> Result<DeleteSecurityConfigurationOutput, DeleteSecurityConfigurationError>;
                

                #[doc="<p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. For information about the cluster steps, see <a>ListSteps</a>.</p>"]
                fn describe_cluster(&self, input: &DescribeClusterInput)  -> Result<DescribeClusterOutput, DescribeClusterError>;
                

                #[doc="<p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>"]
                fn describe_job_flows(&self, input: &DescribeJobFlowsInput)  -> Result<DescribeJobFlowsOutput, DescribeJobFlowsError>;
                

                #[doc="<p>Provides the details of a security configuration by returning the configuration JSON.</p>"]
                fn describe_security_configuration(&self, input: &DescribeSecurityConfigurationInput)  -> Result<DescribeSecurityConfigurationOutput, DescribeSecurityConfigurationError>;
                

                #[doc="<p>Provides more detail about the cluster step.</p>"]
                fn describe_step(&self, input: &DescribeStepInput)  -> Result<DescribeStepOutput, DescribeStepError>;
                

                #[doc="<p>Provides information about the bootstrap actions associated with a cluster.</p>"]
                fn list_bootstrap_actions(&self, input: &ListBootstrapActionsInput)  -> Result<ListBootstrapActionsOutput, ListBootstrapActionsError>;
                

                #[doc="<p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>"]
                fn list_clusters(&self, input: &ListClustersInput)  -> Result<ListClustersOutput, ListClustersError>;
                

                #[doc="<p>Provides all available details about the instance groups in a cluster.</p>"]
                fn list_instance_groups(&self, input: &ListInstanceGroupsInput)  -> Result<ListInstanceGroupsOutput, ListInstanceGroupsError>;
                

                #[doc="<p>Provides information about the cluster instances that Amazon EMR provisions on behalf of a user when it creates the cluster. For example, this operation indicates when the EC2 instances reach the Ready state, when instances become available to Amazon EMR to use for jobs, and the IP addresses for cluster instances, etc.</p>"]
                fn list_instances(&self, input: &ListInstancesInput)  -> Result<ListInstancesOutput, ListInstancesError>;
                

                #[doc="<p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>"]
                fn list_security_configurations(&self, input: &ListSecurityConfigurationsInput)  -> Result<ListSecurityConfigurationsOutput, ListSecurityConfigurationsError>;
                

                #[doc="<p>Provides a list of steps for the cluster in reverse order unless you specify stepIds with the request.</p>"]
                fn list_steps(&self, input: &ListStepsInput)  -> Result<ListStepsOutput, ListStepsError>;
                

                #[doc="<p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>"]
                fn modify_instance_groups(&self, input: &ModifyInstanceGroupsInput)  -> Result<(), ModifyInstanceGroupsError>;
                

                #[doc="<p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>"]
                fn put_auto_scaling_policy(&self, input: &PutAutoScalingPolicyInput)  -> Result<PutAutoScalingPolicyOutput, PutAutoScalingPolicyError>;
                

                #[doc="<p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>"]
                fn remove_auto_scaling_policy(&self, input: &RemoveAutoScalingPolicyInput)  -> Result<RemoveAutoScalingPolicyOutput, RemoveAutoScalingPolicyError>;
                

                #[doc="<p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>"]
                fn remove_tags(&self, input: &RemoveTagsInput)  -> Result<RemoveTagsOutput, RemoveTagsError>;
                

                #[doc="<p>RunJobFlow creates and starts running a new job flow. The job flow will run the steps specified. After the job flow completes, the cluster is stopped and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the job flow will transition to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the job flow and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your job flow is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/Management/Guide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Job Flow</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running job flows, we recommend that you periodically store your results.</p>"]
                fn run_job_flow(&self, input: &RunJobFlowInput)  -> Result<RunJobFlowOutput, RunJobFlowError>;
                

                #[doc="<p>SetTerminationProtection locks a job flow so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling SetTerminationProtection on a job flow is analogous to calling the Amazon EC2 DisableAPITermination API on all of the EC2 instances in a cluster.</p> <p>SetTerminationProtection is used to prevent accidental termination of a job flow and to ensure that in the event of an error, the instances will persist so you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a job flow that has been locked by setting SetTerminationProtection to <code>true</code>, you must first unlock the job flow by a subsequent call to SetTerminationProtection in which you set the value to <code>false</code>. </p> <p> For more information, see<a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/UsingEMR_TerminationProtection.html\">Protecting a Job Flow from Termination</a> in the <i>Amazon EMR Guide.</i> </p>"]
                fn set_termination_protection(&self, input: &SetTerminationProtectionInput)  -> Result<(), SetTerminationProtectionError>;
                

                #[doc="<p>Sets whether all AWS Identity and Access Management (IAM) users under your account can access the specified job flows. This action works on running job flows. You can also set the visibility of a job flow when you launch it using the <code>VisibleToAllUsers</code> parameter of <a>RunJobFlow</a>. The SetVisibleToAllUsers action can be called only by an IAM user who created the job flow or the AWS account that owns the job flow.</p>"]
                fn set_visible_to_all_users(&self, input: &SetVisibleToAllUsersInput)  -> Result<(), SetVisibleToAllUsersError>;
                

                #[doc="<p>TerminateJobFlows shuts a list of job flows down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the job flow is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the job flow was created.</p> <p>The maximum number of JobFlows allowed is 10. The call to TerminateJobFlows is asynchronous. Depending on the configuration of the job flow, it may take up to 1-5 minutes for the job flow to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>"]
                fn terminate_job_flows(&self, input: &TerminateJobFlowsInput)  -> Result<(), TerminateJobFlowsError>;
                
}
/// A client for the Amazon EMR API.
        pub struct EmrClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> EmrClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  EmrClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> Emr for EmrClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p>Adds one or more instance groups to a running cluster.</p>"]
                fn add_instance_groups(&self, input: &AddInstanceGroupsInput)  -> Result<AddInstanceGroupsOutput, AddInstanceGroupsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.AddInstanceGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddInstanceGroupsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AddInstanceGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>AddJobFlowSteps adds new steps to a running job flow. A maximum of 256 steps are allowed in each job flow.</p> <p>If your job flow is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Job Flow</a> in the <i>Amazon EMR Developer's Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the job flow or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a job flow that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>"]
                fn add_job_flow_steps(&self, input: &AddJobFlowStepsInput)  -> Result<AddJobFlowStepsOutput, AddJobFlowStepsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.AddJobFlowSteps");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddJobFlowStepsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AddJobFlowStepsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
                fn add_tags(&self, input: &AddTagsInput)  -> Result<AddTagsOutput, AddTagsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.AddTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddTagsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AddTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>"]
                fn cancel_steps(&self, input: &CancelStepsInput)  -> Result<CancelStepsOutput, CancelStepsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.CancelSteps");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CancelStepsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CancelStepsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>"]
                fn create_security_configuration(&self, input: &CreateSecurityConfigurationInput)  -> Result<CreateSecurityConfigurationOutput, CreateSecurityConfigurationError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.CreateSecurityConfiguration");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateSecurityConfigurationOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateSecurityConfigurationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a security configuration.</p>"]
                fn delete_security_configuration(&self, input: &DeleteSecurityConfigurationInput)  -> Result<DeleteSecurityConfigurationOutput, DeleteSecurityConfigurationError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.DeleteSecurityConfiguration");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteSecurityConfigurationOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteSecurityConfigurationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. For information about the cluster steps, see <a>ListSteps</a>.</p>"]
                fn describe_cluster(&self, input: &DescribeClusterInput)  -> Result<DescribeClusterOutput, DescribeClusterError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.DescribeCluster");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeClusterOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeClusterError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>"]
                fn describe_job_flows(&self, input: &DescribeJobFlowsInput)  -> Result<DescribeJobFlowsOutput, DescribeJobFlowsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.DescribeJobFlows");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeJobFlowsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeJobFlowsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides the details of a security configuration by returning the configuration JSON.</p>"]
                fn describe_security_configuration(&self, input: &DescribeSecurityConfigurationInput)  -> Result<DescribeSecurityConfigurationOutput, DescribeSecurityConfigurationError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.DescribeSecurityConfiguration");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeSecurityConfigurationOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeSecurityConfigurationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides more detail about the cluster step.</p>"]
                fn describe_step(&self, input: &DescribeStepInput)  -> Result<DescribeStepOutput, DescribeStepError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.DescribeStep");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStepOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeStepError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides information about the bootstrap actions associated with a cluster.</p>"]
                fn list_bootstrap_actions(&self, input: &ListBootstrapActionsInput)  -> Result<ListBootstrapActionsOutput, ListBootstrapActionsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListBootstrapActions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListBootstrapActionsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListBootstrapActionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>"]
                fn list_clusters(&self, input: &ListClustersInput)  -> Result<ListClustersOutput, ListClustersError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListClusters");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListClustersOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListClustersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides all available details about the instance groups in a cluster.</p>"]
                fn list_instance_groups(&self, input: &ListInstanceGroupsInput)  -> Result<ListInstanceGroupsOutput, ListInstanceGroupsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListInstanceGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListInstanceGroupsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListInstanceGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides information about the cluster instances that Amazon EMR provisions on behalf of a user when it creates the cluster. For example, this operation indicates when the EC2 instances reach the Ready state, when instances become available to Amazon EMR to use for jobs, and the IP addresses for cluster instances, etc.</p>"]
                fn list_instances(&self, input: &ListInstancesInput)  -> Result<ListInstancesOutput, ListInstancesError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListInstancesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>"]
                fn list_security_configurations(&self, input: &ListSecurityConfigurationsInput)  -> Result<ListSecurityConfigurationsOutput, ListSecurityConfigurationsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListSecurityConfigurations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListSecurityConfigurationsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListSecurityConfigurationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides a list of steps for the cluster in reverse order unless you specify stepIds with the request.</p>"]
                fn list_steps(&self, input: &ListStepsInput)  -> Result<ListStepsOutput, ListStepsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ListSteps");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListStepsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListStepsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>"]
                fn modify_instance_groups(&self, input: &ModifyInstanceGroupsInput)  -> Result<(), ModifyInstanceGroupsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.ModifyInstanceGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(ModifyInstanceGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>"]
                fn put_auto_scaling_policy(&self, input: &PutAutoScalingPolicyInput)  -> Result<PutAutoScalingPolicyOutput, PutAutoScalingPolicyError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.PutAutoScalingPolicy");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutAutoScalingPolicyOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutAutoScalingPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>"]
                fn remove_auto_scaling_policy(&self, input: &RemoveAutoScalingPolicyInput)  -> Result<RemoveAutoScalingPolicyOutput, RemoveAutoScalingPolicyError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.RemoveAutoScalingPolicy");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveAutoScalingPolicyOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RemoveAutoScalingPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>"]
                fn remove_tags(&self, input: &RemoveTagsInput)  -> Result<RemoveTagsOutput, RemoveTagsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.RemoveTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTagsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RemoveTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>RunJobFlow creates and starts running a new job flow. The job flow will run the steps specified. After the job flow completes, the cluster is stopped and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the job flow will transition to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the job flow and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your job flow is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/Management/Guide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Job Flow</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running job flows, we recommend that you periodically store your results.</p>"]
                fn run_job_flow(&self, input: &RunJobFlowInput)  -> Result<RunJobFlowOutput, RunJobFlowError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.RunJobFlow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RunJobFlowOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RunJobFlowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>SetTerminationProtection locks a job flow so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling SetTerminationProtection on a job flow is analogous to calling the Amazon EC2 DisableAPITermination API on all of the EC2 instances in a cluster.</p> <p>SetTerminationProtection is used to prevent accidental termination of a job flow and to ensure that in the event of an error, the instances will persist so you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a job flow that has been locked by setting SetTerminationProtection to <code>true</code>, you must first unlock the job flow by a subsequent call to SetTerminationProtection in which you set the value to <code>false</code>. </p> <p> For more information, see<a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/UsingEMR_TerminationProtection.html\">Protecting a Job Flow from Termination</a> in the <i>Amazon EMR Guide.</i> </p>"]
                fn set_termination_protection(&self, input: &SetTerminationProtectionInput)  -> Result<(), SetTerminationProtectionError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.SetTerminationProtection");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(SetTerminationProtectionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Sets whether all AWS Identity and Access Management (IAM) users under your account can access the specified job flows. This action works on running job flows. You can also set the visibility of a job flow when you launch it using the <code>VisibleToAllUsers</code> parameter of <a>RunJobFlow</a>. The SetVisibleToAllUsers action can be called only by an IAM user who created the job flow or the AWS account that owns the job flow.</p>"]
                fn set_visible_to_all_users(&self, input: &SetVisibleToAllUsersInput)  -> Result<(), SetVisibleToAllUsersError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.SetVisibleToAllUsers");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(SetVisibleToAllUsersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>TerminateJobFlows shuts a list of job flows down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the job flow is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the job flow was created.</p> <p>The maximum number of JobFlows allowed is 10. The call to TerminateJobFlows is asynchronous. Depending on the configuration of the job flow, it may take up to 1-5 minutes for the job flow to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>"]
                fn terminate_job_flows(&self, input: &TerminateJobFlowsInput)  -> Result<(), TerminateJobFlowsError> {
                    let mut request = SignedRequest::new("POST", "elasticmapreduce", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "ElasticMapReduce.TerminateJobFlows");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(TerminateJobFlowsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
