use teximage2d;


#[test]
fn test_tex_image_2d_code_generator_end_to_end() {
    let expected = teximage2d::load_file(SAMPLE_DATA).unwrap();
    let result = include!("sample_png_test.in");

    assert_eq!(result, expected);
}
