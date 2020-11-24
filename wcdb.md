# WCDB Overview - Part 1
In this article, we will take a close look at [WCDB](https://github.com/Tencent/wcdb), an efficient, complete, easy-to-use mobile database framework built by WeChat. We focus mainly on it's ORM and WINQ subsystem, by digging into the source code, trying to understand how it is implemented.

## WCDB Usage
To use WCDB create table in database, we should first define our business model, e.g.
```objc
@interface Employee : NSObject
@property (nonatomic, assign) int ID;
@property (nonatomic, strong) NSString *name;
@property (nonatomic, assign) int age;
@end

//WCTTableCoding
@interface Employee () <WCTTableCoding>
WCDB_PROPERTY(ID)
WCDB_PROPERTY(name)
WCDB_PROPERTY(age)
@end


@implementation Employee

WCDB_IMPLEMENTATION(Employee)

WCDB_SYNTHESIZE(Employee, ID)
WCDB_SYNTHESIZE(Employee, name)
WCDB_SYNTHESIZE(Employee, age)

WCDB_PRIMARY_ASC_AUTO_INCREMENT(Employee, ID)

- (NSString *)description {
    return [NSString stringWithFormat:@"<Employee>: %d %@ %d", self.ID, self.name, self.age];
}
@end

```
Note that out biz model should conforms to `WCTTableCoding` protocol, actually it's automatically synthesized by the macro `WCDB_PROPERTY`, `WCDB_SYNTHESIZE`.

```objc
@protocol WCTTableCoding
@required
+ (const WCTBinding *)objectRelationalMappingForWCDB;
+ (const WCTPropertyList &)AllProperties;
+ (const WCTAnyProperty &)AnyProperty;
+ (WCTPropertyNamed)PropertyNamed; //className.PropertyNamed(propertyName)
@optional
@property(nonatomic, assign) long long lastInsertedRowID;
@property(nonatomic, assign) BOOL isAutoIncrement;
@end
```
After that, we can create our table based on our model definition

```objc
WCTDatabase *database = [[WCTDatabase alloc] initWithPath:path]
BOOL ret =  [database createTableAndIndexesOfName:tableName withClass:Employee.class];
```

### CRUD
WCDB make CRUD operation quite easy, take out `Employee` table as an example
1. insert
    ```objc
      { // insert objs
        
        Employee *e = [Employee new];
        e.name = @"Paul";
        e.age = 32;
        e.isAutoIncrement = YES;
        
        Employee *e1 = [Employee new];
        e1.name = @"Allen";
        e1.age = 25;
        e1.isAutoIncrement = YES;
        
        
        WCTInsert *insert  = [database prepareInsertObjectsOfClass:Employee.class into:tableName];
        
        
        BOOL ret = [insert executeWithObjects:@[e, e1]];
        if (!ret) {
            NSLog(@"inset objs error: %@", insert.error);
        }
    }
2. select
   ```objc
    { //select objects
        WCTSelect *select = [database prepareSelectObjectsOfClass:Employee.class fromTable:tableName];
        
        NSArray *objects = select.allObjects;
        for (Employee *ee in objects) {
            NSLog(@"%@", ee);
        }
    }
   ```
   output:
   ```
    <Employee>: 1 Paul 32
    <Employee>: 2 Allen 25
   ```
3. update
   ```objc
    { //update by objects
        WCTUpdate *update = [[database prepareUpdateTable:tableName onProperties:Employee.age] where:Employee.name == @"Paul"];
        Employee *ee = [Employee new];
        ee.age = 33;
        BOOL ret = [update executeWithObject:ee];
        if (!ret) {
            NSLog(@"Update by object Error: %@", update.error);
        }
    }
    
   ```
   output:
   ```objc
    <Employee>: 1 Paul 33
    <Employee>: 2 Allen 25
   ```
4. delete 
    ```objc
    
    { //update by objects
        WCTDelete *deletion = [[database prepareDeleteFromTable:tableName] where:Employee.name == @"Paul"];
        BOOL ret = [deletion execute];
        if (!ret) {
            NSLog(@"Delete Error %@", deletion.error);
        }
    }
    ```
    output:
    ```objc
    <Employee>: 2 Allen 25
    ```

## ORM 
ORM (Object Relational Mapping), ORM is actually the process of binding, binding our biz domain model to our database model, the automatic binding takes place inside the macros we see before `WCDB_IMPLEMENTATION`, `WCDB_SYNTHESIZE`, `WCDB_PRIMARY_ASC_AUTO_INCREMENT`, `WCDB_PROPERTY`.   What does our `Employee` model looks like when we preprocessed these macros? 
```objc
@interface Employee () <WCTTableCoding>
+ (const WCTProperty &)ID;
+ (const WCTProperty &)name;
+ (const WCTProperty &)age;
@end


@implementation Employee

static WCTBinding _s_Employee_binding(Employee.class);
static WCTPropertyList _s_Employee_properties;
+ (const WCTBinding *)objectRelationalMappingForWCDB {
  if (self.class != Employee.class) {
    WCDB::Error::Abort("Inheritance is not supported for ORM");
  }
  return &_s_Employee_binding;
}
+ (const WCTPropertyList &)AllProperties {
  return _s_Employee_properties;
}
+ (const WCTAnyProperty &)AnyProperty {
  static const WCTAnyProperty s_anyProperty(Employee.class);
  return s_anyProperty;
}
+ (WCTPropertyNamed)PropertyNamed {
  return WCTProperty::PropertyNamed;
}

+ (const WCTProperty &)ID {
  static const WCTProperty s_property(
      "ID", Employee.class,
      _s_Employee_binding.addColumnBinding<decltype([Employee new].ID)>("ID",
                                                                        "ID"));
  return s_property;
}
static const auto _unused0 = [](WCTPropertyList &propertyList) {
  propertyList.push_back(Employee.ID);
  return nullptr;
}(_s_Employee_properties);
+ (const WCTProperty &)name {
  static const WCTProperty s_property(
      "name", Employee.class,
      _s_Employee_binding.addColumnBinding<decltype([Employee new].name)>(
          "name", "name"));
  return s_property;
}
static const auto _unused1 = [](WCTPropertyList &propertyList) {
  propertyList.push_back(Employee.name);
  return nullptr;
}(_s_Employee_properties);
+ (const WCTProperty &)age {
  static const WCTProperty s_property(
      "age", Employee.class,
      _s_Employee_binding.addColumnBinding<decltype([Employee new].age)>(
          "age", "age"));
  return s_property;
}
static const auto _unused2 = [](WCTPropertyList &propertyList) {
  propertyList.push_back(Employee.age);
  return nullptr;
}(_s_Employee_properties);

@synthesize isAutoIncrement;
@synthesize lastInsertedRowID;
static const auto _unused3 = [](WCTBinding *binding) {
  binding->getColumnBinding(Employee.ID)
      ->makePrimary(WCTOrderedAscending, true, WCTConflictNotSet);
  return nullptr;
}(&_s_Employee_binding);

- (NSString *)description {
  return [NSString
      stringWithFormat:@"<Employee>: %d %@ %d", self.ID, self.name, self.age];
}

@end

```
WCDB creates the table binding and column properties behind the scene.



## WINQ 
WINQ(WCDB language integrated query). WINQ is quite similar to LINQ in .net platform. With WINQ, we don't need to write glue code to concat sql query strings. Let's take a deep look at our `delete` operation, to figure out how it is implemented behind  the scene.
```objc
{ //update by objects
    WCTUpdate *update = [[database prepareUpdateTable:tableName onProperties:Employee.age] where:Employee.name == @"Paul"];
    Employee *ee = [Employee new];
    ee.age = 33;
    BOOL ret = [update executeWithObject:ee];
    if (!ret) {
        NSLog(@"Update by object Error: %@", update.error);
    }
}
```

```objc
[[database prepareUpdateTable:tableName onProperties:Employee.age] where:Employee.name == @"Paul"];
```
Create the `WCTUpdate` statement which instantiated with `WCTPropertyList` , in this case, `Employee.age`. As we shown before, `Employee.age` is actually a `WCTProperty` generated by the macro `WCDB_SYNTHESIZE`.

```objc
+ (const WCTProperty &)age {
  static const WCTProperty s_property(
      "age", Employee.class,
      _s_Employee_binding.addColumnBinding<decltype([Employee new].age)>(
          "age", "age"));
  return s_property;
}
```
The `where` clause `Employee.name == @"Paul"` builds the `WCTExpr`, combined with the `where` condition, what we actually get is
```objc
StatementUpdate &StatementUpdate::where(const Expr &where)
{
    if (!where.isEmpty()) {
        m_description.append(" WHERE " + where.getDescription());
    }
    return *this;
}
```
`m_description` is actually the serialized SQL query string. `where` expr get serialized into 
`(age == 33)`.

`[deletion execute]` makes the final execution call
```objc
- (BOOL)execute
{
    WCDB::ScopedTicker scopedTicker(_ticker);
    WCDB::RecyclableStatement statementHandle = _core->prepare(_statement, _error);
    if (!statementHandle) {
        return NO;
    }
    statementHandle->step();
    if (!statementHandle->isOK()) {
        _error = statementHandle->getError();
        return NO;
    }
    _changes = statementHandle->getChanges();
    return YES;
}
```
The `_core->prepare(_statement, _error)` creates the desired sqlite prepared statement
```objc
std::shared_ptr<StatementHandle> Handle::prepare(const Statement &statement)
{
    if (statement.getStatementType() == Statement::Type::Transaction) {
        Error::Abort(
            "[prepare] a transaction is not allowed, use [exec] instead",
            &m_error);
        return nullptr;
    }
    sqlite3_stmt *stmt = nullptr;
    int rc = sqlite3_prepare_v2((sqlite3 *) m_handle,
                                statement.getDescription().c_str(), -1, &stmt,
                                nullptr);
    if (rc == SQLITE_OK) {
        m_error.reset();
        return std::shared_ptr<StatementHandle>(
            new StatementHandle(stmt, *this));
    }
    Error::ReportSQLite(m_tag, path, Error::HandleOperation::Prepare, rc,
                        sqlite3_extended_errcode((sqlite3 *) m_handle),
                        sqlite3_errmsg((sqlite3 *) m_handle),
                        statement.getDescription(), &m_error);
    return nullptr;
}
```
The `step` and is an thin wrapper of `sqlite3_step`.
```objc
bool StatementHandle::step()
{
    int rc = sqlite3_step((sqlite3_stmt *) m_stmt);
    if (rc == SQLITE_ROW || rc == SQLITE_OK || rc == SQLITE_DONE) {
        m_error.reset();
        return rc == SQLITE_ROW;
    }
    sqlite3 *handle = sqlite3_db_handle((sqlite3_stmt *) m_stmt);
    Error::ReportSQLite(
        m_handle.getTag(), m_handle.path, Error::HandleOperation::Step, rc,
        sqlite3_extended_errcode(handle), sqlite3_errmsg(handle),
        sqlite3_sql((sqlite3_stmt *) m_stmt), &m_error);
    return false;
}
```
We have walk through the entire process of WINQ operation. The whole process chain can be summarized as 
```
WCTUpdate -> Serialization -> Prepared Statement -> Execution
```
The other `CRUD` operations are quite similar, WCDB provides `WCTInset`, `WCTSelect`, `WCTDelete`, etc. for CURD chain call. We can build complex SQL queries without too much effort.

## Insights
WCDB provides ORM & WINQ facilities to provide easy manipulation of underlying sqlite database. Actually provides much more capabilities such as multi-thread concurrency access, encryption, corruption recovery, and anti-injection. We will discuss this in the following series.