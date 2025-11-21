# Mermaid 图表语法指南

## 1. 流程图 (Flowchart)

### 基础语法
- `graph TD` - 从上到下
- `graph LR` - 从左到右
- `graph BT` - 从下到上  
- `graph RL` - 从右到左

### 示例

```mermaid
graph TD
    A[方形节点] --> B(圆角节点)
    B --> C{菱形节点}
    C -->|选项1| D[结果1]
    C -->|选项2| E[结果2]
```

### 节点形状
- `[方形]` 矩形
- `(圆角)` 圆角矩形
- `{菱形}` 菱形/判断
- `((圆形))` 圆形
- `>不规则]` 旗帜形

```mermaid
graph LR
    A[矩形] --> B(圆角矩形)
    B --> C{菱形}
    C --> D((圆形))
    D --> E>旗帜]
```

## 2. 序列图 (Sequence Diagram)

### 基础语法
- `participant` - 定义参与者
- `->>` - 实线箭头
- `-->>` - 虚线箭头
- `-x` - 结束线
- `autonumber` - 自动编号

### 示例

```mermaid
sequenceDiagram
    autonumber
    Alice->>John: 你好 John
    John-->>Alice: 你好 Alice
    Alice->>John: 最近怎么样？
    John-->>Alice: 很好！
```

### 带循环和条件

```mermaid
sequenceDiagram
    participant A as 用户
    participant B as 系统
    
    A->>B: 发送请求
    
    alt 成功
        B-->>A: 返回数据
    else 失败
        B-->>A: 错误信息
    end
    
    loop 每分钟
        B->>B: 检查状态
    end
```

## 3. 类图 (Class Diagram)

### 基础语法
- `+` 公有
- `-` 私有
- `#` 受保护
- `<|--` 继承
- `*--` 组合
- `o--` 聚合

### 示例

```mermaid
classDiagram
    class Animal {
        +String name
        +int age
        +eat()
        +sleep()
    }
    
    class Dog {
        +String breed
        +bark()
    }
    
    class Cat {
        +meow()
    }
    
    Animal <|-- Dog
    Animal <|-- Cat
```

## 4. 状态图 (State Diagram)

### 示例

```mermaid
stateDiagram-v2
    [*] --> 待办
    待办 --> 进行中: 开始
    进行中 --> 已完成: 完成
    进行中 --> 待办: 暂停
    已完成 --> [*]
```

## 5. 甘特图 (Gantt Chart)

### 示例

```mermaid
gantt
    title 项目计划
    dateFormat YYYY-MM-DD
    section 阶段1
    需求分析 :a1, 2024-01-01, 7d
    设计     :a2, after a1, 5d
    section 阶段2
    开发     :a3, after a2, 15d
    测试     :a4, after a3, 5d
```

## 6. 饼图 (Pie Chart)

### 示例

```mermaid
pie title 编程语言使用占比
    "JavaScript" : 40
    "Python" : 30
    "Java" : 15
    "其他" : 15
```

## 7. 实体关系图 (ER Diagram)

### 示例

```mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
    CUSTOMER {
        string name
        string email
        int id
    }
    ORDER {
        int id
        date orderDate
    }
    LINE-ITEM {
        int quantity
        decimal price
    }
```

## 8. Git 图 (Git Graph)

### 示例

```mermaid
gitGraph
    commit
    commit
    branch develop
    checkout develop
    commit
    commit
    checkout main
    merge develop
    commit
```

## 9. 用户旅程图 (User Journey)

### 示例

```mermaid
journey
    title 用户购物旅程
    section 浏览
      访问网站: 5: 用户
      搜索商品: 4: 用户
    section 购买
      添加到购物车: 3: 用户
      结账: 2: 用户
      支付: 5: 用户
    section 评价
      收到商品: 5: 用户
      留下评价: 4: 用户
```

## 10. 时间轴 (Timeline)

### 示例

```mermaid
timeline
    title 产品发展历史
    2020 : 项目启动
         : 团队组建
    2021 : 发布 v1.0
         : 获得首批用户
    2022 : 功能扩展
         : 用户突破 10万
    2023 : 发布 v2.0
         : 企业版推出
```

## 常见问题

### 1. 中文支持
Mermaid 完全支持中文，直接使用即可。

### 2. 主题
系统会自动根据深色/浅色模式切换主题。

### 3. 调试
如果图表不显示，检查：
- 语法是否正确
- 代码块标记是否为 \`\`\`mermaid
- 浏览器控制台是否有错误

### 4. 箭头类型
- `-->` 实线箭头
- `-.->` 虚线箭头  
- `==>` 粗箭头
- `->>` 序列图箭头
- `-->>` 序列图虚线箭头
