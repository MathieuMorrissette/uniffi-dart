import 'package:test/test.dart';
import '../trait_interfaces.dart';

void main() {
  group('FriendlyGreeter', () {
    test('implements generated interface', () {
      final greeter = FriendlyGreeter('Hello');
      expect(greeter is FriendlyGreeterInterface, isTrue);

      final FriendlyGreeterInterface iface = greeter;
      expect(iface.greet('World'), equals('Hello World'));
    });

    test('equality and hashing honour trait implementations', () {
      final a = FriendlyGreeter('Hi');
      final b = FriendlyGreeter('Hi');
      final c = FriendlyGreeter('Hey');

      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
      expect(a, isNot(equals(c)));

      final map = <FriendlyGreeter, String>{a: 'value'};
      expect(map.containsKey(b), isTrue);
    });
  });

  group('ProcFriendlyGreeter', () {
    test('proc-macro object honours traits and methods', () {
      final greeter = ProcFriendlyGreeter('hola');
      expect(greeter.greet('mundo'), equals('HOLA MUNDO'));
      expect(greeter.toString(), equals('ProcFriendlyGreeter(hola)'));
    });
  });
}
