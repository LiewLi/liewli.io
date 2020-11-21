# SQLite ORM

How can we build a elegant ORM wrapper for sqlite database? 

ORM involves designing an easy-to-use model mapping system, LINQ system and much more. Let's take a look at [sqlite_orm](https://github.com/fnc12/sqlite_orm), trying to investigate it's whole architecture and implementation details, see how it brings us the answer to the question. This article assume you have a basic knowledge of SQLite and modern c++ (C++14 features is heavily used in sqlite_orm).

## API design

### Model Mapping

Data model definition and it's corresponding mapping
```cpp
struct Employee {
    int id;
    std::string name;
    int age;
    std::unique_ptr<std::string> address;  //  optional
    std::unique_ptr<double> salary;  //  optional
};
```

``` cpp
auto storage = make_storage("select.sqlite",
                                make_table("COMPANY",
                                           make_column("ID", &Employee::id, primary_key()),
                                           make_column("NAME", &Employee::name),
                                           make_column("AGE", &Employee::age),
                                           make_column("ADDRESS", &Employee::address),
                                           make_column("SALARY", &Employee::salary)));
```

### CRUD
Let's mock some data
```cpp
//  create employees..
Employee paul{-1, "Paul", 32, std::make_unique<std::string>("California"), std::make_unique<double>(20000.0)};
Employee allen{-1, "Allen", 25, std::make_unique<std::string>("Texas"), std::make_unique<double>(15000.0)};
Employee teddy{-1, "Teddy", 23, std::make_unique<std::string>("Norway"), std::make_unique<double>(20000.0)};
Employee mark{-1, "Mark", 25, std::make_unique<std::string>("Rich-Mond"), std::make_unique<double>(65000.0)};
Employee david{-1, "David", 27, std::make_unique<std::string>("Texas"), std::make_unique<double>(85000.0)};
Employee kim{-1, "Kim", 22, std::make_unique<std::string>("South-Hall"), std::make_unique<double>(45000.0)};
Employee james{-1, "James", 24, std::make_unique<std::string>("Houston"), std::make_unique<double>(10000.0)};
```

Add
```cpp
Employee paul{-1, "Paul", 32, std::make_unique<std::string>("California"), std::make_unique<double>(20000.0)};
paul.id = storage.insert(paul);
```

Delete
```cpp
storage.remove<Employee>(paul.id);
```

Update
```cpp
allen.age = 30;
storage.update<Employee>(allen);
```

Select
```cpp
//e == allen
auto e = storage.get<Employee>(allen.id);
```

## Implementation
Let's dive into `storage.remove<Employee>(paul.id)` implementation detail, how it is translated into corresponding SQL query and gets executed. Actually it is processed by the following pipeline:
```
remove_t statement -> serialization -> prepared statement -> statement binding -> execute
```
1. create remove_t statement to capture all params, remove_t is defined as:
```cpp
template<class T, class... Ids>
struct remove_t {
   using type = T;
   using ids_type = std::tuple<Ids...>;

   ids_type ids;
};
```
* *T* is the table model type *Employee*, *ids* is the primary key 


2. statement serialization, the serialization process is pretty straightforward

```cpp
template<class T, class... Ids>
struct statement_serializator<remove_t<T, Ids...>, void> {
   using statement_type = remove_t<T, Ids...>;

   template<class C>
   std::string operator()(const statement_type &, const C &context) const {
       auto &tImpl = context.impl.template get_impl<T>();
       std::stringstream ss;
       ss << "DELETE FROM '" << tImpl.table.name << "' ";
       ss << "WHERE ";
       auto primaryKeyColumnNames = tImpl.table.primary_key_column_names();
       for(size_t i = 0; i < primaryKeyColumnNames.size(); ++i) {
           ss << "\"" << primaryKeyColumnNames[i] << "\""
              << " = ? ";
           if(i < primaryKeyColumnNames.size() - 1) {
               ss << "AND ";
           }
       }
       return ss.str();
   }
};
```

3. created sqlite prepared statement, calling `sqlite3_prepare_v2` to make the prepared statement
```cpp
template<class T, class... Ids>
prepared_statement_t<remove_t<T, Ids...>> prepare(remove_t<T, Ids...> rem) {
    auto con = this->get_connection();
    sqlite3_stmt *stmt;
    auto db = con.get();
    using context_t = serializator_context<impl_type>;
    context_t context{this->impl};
    context.skip_table_name = false;
    context.replace_bindable_with_question = true;
    auto query = serialize(rem, context);
    if(sqlite3_prepare_v2(db, query.c_str(), -1, &stmt, nullptr) == SQLITE_OK) {
        return {std::move(rem), stmt, con};
    } else {
        throw std::system_error(std::error_code(sqlite3_errcode(db), get_sqlite_error_category()),
                                sqlite3_errmsg(db));
    }
}
```

4. statement binding and execution
```cpp
template<class T, class... Ids>
void execute(const prepared_statement_t<remove_t<T, Ids...>> &statement) {
    auto con = this->get_connection();
    auto db = con.get();
    auto stmt = statement.stmt;
    auto index = 1;
    sqlite3_reset(stmt);
    iterate_ast(statement.t.ids, [stmt, &index, db](auto &v) {
        using field_type = typename std::decay<decltype(v)>::type;
        if(SQLITE_OK != statement_binder<field_type>().bind(stmt, index++, v)) {
            throw std::system_error(std::error_code(sqlite3_errcode(db), get_sqlite_error_category()),
                                    sqlite3_errmsg(db));
        }
    });
    if(sqlite3_step(stmt) == SQLITE_DONE) {
        //  done..
    } else {
        throw std::system_error(std::error_code(sqlite3_errcode(db), get_sqlite_error_category()),
                                sqlite3_errmsg(db));
    }
}
```
In this case `field_type` is *int*, statement_binder do the actual binding as
```cpp
 template<class V>
 struct statement_binder<V, std::enable_if_t<std::is_arithmetic<V>::value>> {
     int bind(sqlite3_stmt *stmt, int index, const V &value) {
         return bind(stmt, index, value, tag());
     }

   private:
     using tag = arithmetic_tag_t<V>;

     int bind(sqlite3_stmt *stmt, int index, const V &value, const int_or_smaller_tag &) {
         return sqlite3_bind_int(stmt, index, static_cast<int>(value));
     }

     int bind(sqlite3_stmt *stmt, int index, const V &value, const bigint_tag &) {
         return sqlite3_bind_int64(stmt, index, static_cast<sqlite3_int64>(value));
     }

     int bind(sqlite3_stmt *stmt, int index, const V &value, const real_tag &) {
         return sqlite3_bind_double(stmt, index, static_cast<double>(value));
     }
 };
```

We have done a through investigation on how LINQ is implemented in sqlite_orm. Finally, let's trying to figure out how model mapping works, the answer lies in
the function call `make_storage` and `make_table` and `make_column`.

### make_column
```cpp
 template<class O,
          class T,
          typename = typename std::enable_if<!std::is_member_function_pointer<T O::*>::value>::type,
          class... Op>
 internal::column_t<O, T, const T &(O::*)() const, void (O::*)(T), Op...>
 make_column(const std::string &name, T O::*m, Op... constraints) {
     static_assert(constraints::template constraints_size<Op...>::value == std::tuple_size<std::tuple<Op...>>::value,
                   "Incorrect constraints pack");
     static_assert(internal::is_field_member_pointer<T O::*>::value,
                   "second argument expected as a member field pointer, not member function pointer");
     return {name, m, nullptr, nullptr, std::make_tuple(constraints...)};
 }
```
column_t is defined as:
```cpp
template<class O, class T, class G /* = const T& (O::*)() const*/, class S /* = void (O::*)(T)*/, class... Op>
struct column_t : column_base {
   using object_type = O;
   using field_type = T;
   using constraints_type = std::tuple<Op...>;
   using member_pointer_t = field_type object_type::*;
   using getter_type = G;
   using setter_type = S;

   /**
    *  Member pointer used to read/write member
    */
   member_pointer_t member_pointer /* = nullptr*/;

   /**
    *  Getter member function pointer to get a value. If member_pointer is null than
    *  `getter` and `setter` must be not null
    */
   getter_type getter /* = nullptr*/;

   /**
    *  Setter member function
    */
   setter_type setter /* = nullptr*/;

   /**
    *  Constraints tuple
    */
    constraints_type constraints;

    //...
};
```

### make_table
`table_t` is pretty simple, it's just a tuple of `column_t` and a table name

```cpp
struct table_base {

   /**
    *  Table name.
    */
   std::string name;

   bool _without_rowid = false;
};

/**
*  Table interface class. Implementation is hidden in `table_impl` class.
*/
template<class T, class... Cs>
struct table_t : table_base {
   using object_type = T;
   using columns_type = std::tuple<Cs...>;

   static constexpr const int columns_count = static_cast<int>(std::tuple_size<columns_type>::value);

   columns_type columns;
   //...
};
```

### make_storage
`storage_t` plays a key role in sqlite_orm, it is composed of multiple tables and capable of managing connection to the underlying db, provide the facade to whole library etc. We don't take a deeper look for now, since we have grasped the basics of it's capabilities.


## Insights

1. Model mapping and LINQ are two key parts in a ORM library
2. With LINQ, we provide easy-to-use interface to our library user, yet we don't lose it's flexibility, we can build complex SQL queries and validate SQL query statement at the compile time.
3. Modern C++ can be really expressive , we can write fluent code and build complex systems. 
