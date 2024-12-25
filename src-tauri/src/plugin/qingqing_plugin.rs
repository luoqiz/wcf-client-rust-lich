extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::process::Command;

use regex::Regex;
use reqwest::{Client, get};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    id: i64,
    menuId: i64,
    sasId: i64,
    userId: i64,
    userNickname: String,
    userAvatar: String,
    title: String,
    coverPicUrl: String,
    sourceProvince: String,
    sourceCity: String,
    sourceDistrict: String,
    sourceDetailAddress: String,
    destProvince: String,
    destCity: String,
    destDistrict: String,
    destDetailAddress: String,
    applyStartTime: i64,
    applyExpireTime: i64,
    startTime: i64,
    endTime: i64,
    priceMin: i64,
    priceMax: i64,
    memberPriceMin: i64,
    memberPriceMax: i64,
    storeCount: i64,
    remindNotice: String,
    highlights: String,
    addressLongitude: f64,
    addressLatitude: f64,
    commentPubAuthority: String,
    applyAttachmentState: String,
    recommendState: String,
    applyCensorType: String,
    protocolState: String,
    content: String,
    attachmentCount: i64,
    attachmentDesc: String,
    styleClassCount: i64,
    lastSnapshotId: i64,
    optionalField: String,
    mandatoryField: String,
    lastUpdateTime: i64,
    createTime: i64,
    maxApplier: i64,
    state: String,
    orderVirtualAddCount: i64,
    foreignApplyURL: String,
    sort: i64,
    portalCensorState: String,
    lastSubmitCensorTime: i64,
    portalCensorRemark: String,
    priceUnitName: String,
    applyMethodType: String,
    teamApplyMinUserCount: i64,
    teamApplyMaxUserCount: i64,
    applierIdentityCodeUniqueType: String,
    imGroupQRCode: String,
    saleType: String,
    distributionId: i64,
    depositRatio: f64,
    creditPointSetting: String,
    applierLimit: String,
    styleClassSwitch: i64,
    resaleType: String,
    resaleActivity: String,
    resaleCommissionPercent: String,
    resaleCommissionRatio: f64,
    trueApplyExpireTime: i64,
    valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarStyleClassSetting {
    activityId: i64,
    menuId: i64,
    sasId: i64,
    price: i64,
    memberPrice: i64,
    childPrice: i64,
    memberChildPrice: i64,
    startTime: i64,
    endTime: i64,
    days: i64,
    storeCount: i64,
    payType: String,
    orderVirtualAddCount: i64,
    applyDaysBeforeStart: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    activity: Activity,
    statistic: Statistic,
    attachmentList: Option<Vec<String>>,
    styleClassList: Option<String>,
    additionalServiceList: Option<Vec<String>>,
    last10Appliers: Option<String>,
    last10AppliersOffset: Option<i64>,
    categoryNames: Option<String>,
    captain: Option<String>,
    activityState: Option<i64>,
    useStyleClassTimeSelection: Option<bool>,
    supportChildPrice: Option<bool>,
    calendarStyleClassSetting: Option<CalendarStyleClassSetting>,
    extConfig: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootInterface {
    result: Vec<Result>,
    code: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistic {
    activityId: i64,
    sasId: i64,
    menuId: i64,
    orderCount: i64,
    censoringOrderCount: i64,
    unpaidOrderCount: i64,
    paidOrderCount: i64,
    deliverredOrderCount: i64,
    successOrderCount: i64,
    totalApplierCount: i64,
    successApplierCount: i64,
    totalTeamApplierCount: i64,
    successTeamApplierCount: i64,
    paidAmount: f64,
    signOrderCount: i64,
    signApplierCount: i64,
    totalScoreSignCount: i64,
    commentCount: i64,
    totalScoreRecordCount: i64,
    totalGoodStoreRecordCount: i64,
    lastOrderCreateTime: i64,
    lastCommentTime: i64,
    lastScoreRecordTime: i64,
    totalViewCount: i64,
    lastViewerIds: String,
    lastViewerAvatars: String,
    lastViewerNicknames: String,
    lastViewerIPs: String,
    lastSuccessApplierIds: String,
    lastOrderIds: String,
    totalResaleActivityOrderCount: i64,
    totalResaleActivityPaidOrderCount: i64,
    totalResaleCommission: f64,
    summary: String,
    createTime: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityINfo {
    id: i64,
    title: String,
    valid: bool,
    startTime: i64,
    endTime: i64,
    price: i64,
    memberPrice: i64,
    childPrice: i64,
    memberChildPrice: i64,
    totalApplierCount: i64,
    successApplierCount: i64,
    storeCount: i64,
    payType: String,
    fixedAmount: String,
    fixedAmountMaxRange: i64,
    fixedAmountMinRange: i64,
    priceUnitName: String,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // rust code here
    one_day().await?;
    mul_day().await?;
    free_day().await?;
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    Ok(())
}

pub async fn catery(form: [(&str, &str); 5]) -> Vec<Result> {
    // 创建一个reqwest客户端
    let client = Client::new();
    // 发送POST请求
    let url = "http://qingqing.360jlb.cn/m/rest/events";
    let response = client.post(url)
        // .json(&map)
        .form(&form)
        .send()
        .await.unwrap();

    // 检查响应状态码
    if response.status().is_success() {
        // 解析JSON响应
        let data: RootInterface = response.json().await.unwrap();
        return data.result;
    } else {
        println!("Request failed with status: {:?}", response.status());
        panic!("获取数据失败");
    }
}

pub async fn one_day() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut res = vec!["一日线路".to_string()];
    let mut pageIndex = 1;
    let page = pageIndex.clone().to_string();
    let params = &[
        ("page", page.as_str()),
        ("categoryId", ""),
        ("timeCategoryId", ""),
        ("destAddressId", "-1"),
        ("mid", "52628")
    ];

    let mut data: Vec<Result> = catery(*params).await;

    while data.len() > 0 {
        // 可以在这里使用data处理数据
        for r in data {
          res.push(r.activity.title.clone());
          let temp_res = get_activity_info(r.activity.id.to_string().as_str()).await.unwrap();
          res.push(temp_res);
        }
        pageIndex += 1;
        let page = pageIndex.clone().to_string();
        let params = &[
            ("page", page.as_str()),
            ("categoryId", ""),
            ("timeCategoryId", ""),
            ("destAddressId", "-1"),
            ("mid", "52628")
        ];
        data = catery(*params).await;
    }
    Ok(res.join("\n"))
}

pub async fn mul_day() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut res = vec!["多日线路".to_string()];
    let mut pageIndex = 1;
    let page = pageIndex.clone().to_string();
    let params = &[
        ("page", page.as_str()),
        ("categoryId", ""),
        ("timeCategoryId", ""),
        ("destAddressId", "-1"),
        ("mid", "52631")
    ];

    let mut data: Vec<Result> = catery(*params).await;

    while data.len() > 0 {
        // 可以在这里使用data处理数据
        for r in data {
            res.push(r.activity.title.clone());
            let temp_res = get_activity_info(r.activity.id.to_string().as_str()).await.unwrap();
            res.push(temp_res);
        }
        pageIndex = pageIndex + 1;
        let page = pageIndex.clone().to_string();
        let params = &[
            ("page", page.as_str()),
            ("categoryId", ""),
            ("timeCategoryId", ""),
            ("destAddressId", "-1"),
            ("mid", "52631")
        ];
        data = catery(*params).await;
    }
    Ok(res.join("\n"))
}

pub async fn free_day() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut res = vec!["福利路线".to_string()];
    let mut pageIndex = 1;
    let page = pageIndex.clone().to_string();
    let params = &[
        ("page", page.as_str()),
        ("categoryId", ""),
        ("timeCategoryId", ""),
        ("destAddressId", "-1"),
        ("mid", "52629")
    ];

    let mut data: Vec<Result> = catery(*params).await;

    while data.len() > 0 {
        // 可以在这里使用data处理数据
        for r in data {
          res.push(r.activity.title.clone());
          let temp_res = get_activity_info(r.activity.id.to_string().as_str()).await.unwrap();
          res.push(temp_res);
        }
        pageIndex = pageIndex + 1;
        let page = pageIndex.clone().to_string();
        let params = &[
            ("page", page.as_str()),
            ("categoryId", ""),
            ("timeCategoryId", ""),
            ("destAddressId", "-1"),
            ("mid", "52629")
        ];
        data = catery(*params).await;
    }
    Ok(res.join("\n"))
}


fn test_regex(input: String) -> String {
    // 定义正则表达式模式，匹配以 " batches" 开头并以 ";" 结尾的字符串
    let pattern = r" batches(.*)\[(.|\n|\r\n|br)*?\];"; // ^ 表示匹配字符串开头，\s* 匹配零个或多个空格，.* 匹配零个或多个任意字符，;$ 匹配分号结尾

    // 编译正则表达式
    let re = Regex::new(pattern).unwrap();

    let mut res = "".to_string();
    // 进行匹配
    if let Some(mat) = re.find(&*input) {
        let matched_str = mat.as_str();
        let batch_str = matched_str.replace("batches", "").replace("=", "").replace(";", "").replace("'", "\"");
        let _ = batch_str.trim();
        // println!("Matched: {}", batchStr);

        // 使用正则表达式匹配键（key）并添加双引号
        let re = Regex::new(r#"(?P<key>\w+): "#).unwrap();
        let modified_input = re.replace_all(&*batch_str, r#""${key}":"#);

        // 使用serde_json库将字符串解析为Value类型
        let parsed_value: Value = serde_json::from_str(&modified_input).unwrap();

        // 将Value类型转换为包含结构体的Vec
        let mut parsed_json: Vec<ActivityINfo> = Vec::new();
        if parsed_value.is_array() {
            for item in parsed_value.as_array().unwrap() {
                let my_struct: ActivityINfo = serde_json::from_value(item.clone()).unwrap();
                parsed_json.push(my_struct);
            }
        }
        // println!("{:?}", parsed_json); // 打印解析后的JSON数组
        
        for x in parsed_json {
          res = res + &format!("\t\t {:?}  价格{:?} 名额 {:?}/{:?}(余)", x.title, x.price, x.successApplierCount, x.storeCount)+"\n";
        }
    } 
    return  res;
}

pub async fn get_activity_info(id: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    
    // 发送GET请求获取网页内容
    let url = "http://qingqing.360jlb.cn/m/event?id=".to_string() + id; // 将URL替换为你要获取的网页地址
    let mut res = vec![format!("\t 访问链接{:?}",url).to_string()];
    let response = get(url).await?;
    // 定义正则表达式模式，匹配以 " batches" 开头并以 ";" 结尾的字符串
    let pattern = r"batches.*;$"; // ^ 表示匹配字符串开头，\s* 匹配零个或多个空格，.* 匹配零个或多个任意字符，;$ 匹配分号结尾

    // 检查响应状态码
    if response.status().is_success() {
        // 将响应内容解析为HTML
        let body = response.text().await?;
        // println!("{:#?}",body);
        // 编译正则表达式
        Regex::new(pattern).unwrap();
        res.push(test_regex(body));
    } else {
        println!("Request failed with status: {:?}", response.status());
    }

    Ok(res.join("\n"))
}