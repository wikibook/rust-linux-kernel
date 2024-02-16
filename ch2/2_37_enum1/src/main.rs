struct ElementarySchool {
    room: String,
}

struct MiddleSchool {
    teacher: String,
}

struct HighSchool {
    id: i32,
}

enum SchoolKind {
    Elementary(ElementarySchool),
    Middle(MiddleSchool), 
    High(HighSchool)
}

