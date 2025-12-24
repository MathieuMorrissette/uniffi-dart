import 'package:test/test.dart';
import '../proc_macro_pure_uniffi.dart';

void main() {
  group('Record tests', () {
    test('create person', () {
      final person = createPerson('Alice', 30);
      expect(person.name, equals('Alice'));
      expect(person.age, equals(30));
    });

    test('greet person', () {
      final person = createPerson('Bob', 25);
      final greeting = greet(person);
      expect(greeting, equals('Hello, Bob! You are 25 years old.'));
    });
  });

  group('Enum tests', () {
    test('status to string', () {
      expect(statusToString(UserStatus.active), equals('Active'));
      expect(statusToString(UserStatus.inactive), equals('Inactive'));
      expect(statusToString(UserStatus.pending), equals('Pending'));
    });
  });

  group('Object tests', () {
    test('counter creation and get', () {
      final counter = Counter(0);
      expect(counter.getValue(), equals(0));
    });

    test('counter increment', () {
      final counter = Counter(5);
      expect(counter.getValue(), equals(5));
      counter.increment();
      expect(counter.getValue(), equals(6));
      counter.increment();
      expect(counter.getValue(), equals(7));
    });

    test('multiple counters', () {
      final counter1 = Counter(10);
      final counter2 = Counter(20);

      counter1.increment();
      expect(counter1.getValue(), equals(11));
      expect(counter2.getValue(), equals(20));

      counter2.increment();
      counter2.increment();
      expect(counter1.getValue(), equals(11));
      expect(counter2.getValue(), equals(22));
    });
  });
}
